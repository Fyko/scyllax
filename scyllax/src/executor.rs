//! The `scyllax` [`Executor`] processes queries.
use crate::{
    collection::QueryCollection,
    error::ScyllaxError,
    prelude::WriteQuery,
    queries::{Query, ReadQuery},
};
use scylla::{prepared_statement::PreparedStatement, QueryResult, Session, SessionBuilder};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::{mpsc::error::TrySendError, oneshot};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinSet,
};

/// Creates a new [`CachingSession`] and returns it
pub async fn create_session(
    known_nodes: impl IntoIterator<Item = impl AsRef<str>>,
    default_keyspace: Option<impl Into<String>>,
) -> Result<Session, ScyllaxError> {
    let session = SessionBuilder::new()
        .known_nodes(known_nodes)
        .build()
        .await?;

    if let Some(ks) = default_keyspace {
        session.use_keyspace(ks, true).await?;
    }

    Ok(session)
}

pub trait GetPreparedStatement<T: Query> {
    fn get(&self) -> &PreparedStatement;
}

pub trait GetCoalescingSender<T: ReadQuery> {
    fn get(&self) -> &Sender<ShardMessage<T>>;
}

/// A [`Session`] and a collection of prepared statements.
///
/// The [`Executor`] is responsible for executing queries.
///
/// It coalesces read queries, adding waiters to a pending request. Write queries are executed immediately.
#[derive(Debug, Clone)]
pub struct Executor<T> {
    pub session: Arc<Session>,
    queries: T,
}

/// A message sent to the [`Executor::read_task`] task.
pub type ShardMessage<Q> = (Q, oneshot::Sender<ReadQueryResult<Q>>);

/// The local HashMap of requests being coalesced in a read task.
type TaskRequestMap<Q> = HashMap<u64, Vec<oneshot::Sender<ReadQueryResult<Q>>>>;

/// The result of a read query.
type ReadQueryResult<Q> = Arc<Result<<Q as ReadQuery>::Output, ScyllaxError>>;

/// A message sent to the [`Executor::read_query_runner`] task.
pub struct QueryRunnerMessage<Q: ReadQuery> {
    pub hash: u64,
    pub query: Q,
    pub response_transmitter: oneshot::Sender<ReadQueryResult<Q>>,
}

impl<T: QueryCollection + Clone> Executor<T> {
    /// Creates a new [`Executor`] from a [`Session`] and a [`QueryCollection`].
    // all this is super ugly and inefficient, but its okay because
    // it only happens once per executor
    pub async fn new(session: Arc<Session>) -> Result<Self, ScyllaxError> {
        let queries = T::new(&session).await?;
        let executor = Arc::new(Self {
            session: session.clone(),
            queries,
        });

        let queries = executor.queries.clone().register_tasks(executor);
        let executor = Self { session, queries };

        Ok(executor)
    }

    /// Executes a read query and returns the result.
    pub async fn execute_read<Q>(&self, query: Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let to_coalesce = Q::coalesce();
        if !to_coalesce {
            return self.perform_read_query(query).await;
        }

        let (tx, rx) = oneshot::channel();
        let task = self.queries.get_task::<Q>();

        match task.send((query, tx)).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("error sending query to task: {:#?}", e);
                return Err(ScyllaxError::NoRowsFound);
            }
        }

        let result = match rx.await {
            Ok(result) => result,
            Err(e) => return Err(ScyllaxError::ReceiverError(e)),
        };

        match Arc::try_unwrap(result) {
            Ok(data) => data,
            Err(arc) => (*arc).clone(),
        }
    }

    /// ## internal
    /// the read task is responsible for coalescing requests
    pub async fn read_task<Q>(
        &self,
        request_receiver: Receiver<ShardMessage<Q>>,
        query_runner: Sender<QueryRunnerMessage<Q>>,
    ) where
        Q: ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        run_read_task(request_receiver, query_runner).await;
    }

    /// ## internal
    ///
    /// This function is repsonsible for receiving query requests, executing them, and sending the result back to the requestor.
    ///
    /// It is spawned by the branch of [`Executor::read_task`] that is responsible for coalescing requests.
    pub async fn read_query_runner<Q>(&self, query_receiver: Receiver<QueryRunnerMessage<Q>>)
    where
        Q: Query + ReadQuery + Hash + Send + Sync,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        run_query_runner(query_receiver, |query| self.perform_read_query(query)).await;
    }

    /// ## internal
    ///
    /// Executes a read query and returns the result.
    pub(self) async fn perform_read_query<Q>(&self, query: Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: Query + ReadQuery + Hash + Send + Sync,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();
        // let variables = query.bind().unwrap();
        let response = match self.session.execute(statement, query).await {
            Ok(response) => {
                tracing::debug!(
                    "query executed successfully: {:?} rows",
                    response.rows_num()
                );
                response
            }
            Err(e) => {
                tracing::error!("error executing query: {:#?}", e);
                return Err(e.into());
            }
        };

        Q::parse_response(response).await
    }

    /// Executes a write query and returns the [`scylla::QueryResult`].
    pub async fn execute_write<Q>(&self, query: Q) -> Result<QueryResult, ScyllaxError>
    where
        Q: WriteQuery,
        T: GetPreparedStatement<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();

        self.session
            .execute(statement, query)
            .await
            .map_err(Into::into)
    }
}

async fn run_read_task<Q>(
    mut request_receiver: Receiver<ShardMessage<Q>>,
    query_runner: Sender<QueryRunnerMessage<Q>>,
) where
    Q: ReadQuery,
{
    let mut join_set: JoinSet<_> = JoinSet::new();
    let query_runner = Arc::new(query_runner);

    let mut requests: TaskRequestMap<Q> = HashMap::new();
    loop {
        tokio::select! {
            Some((query, tx)) = request_receiver.recv() => {
                tracing::debug!("recieved a query: {:#?}", query);
                let query_type = std::any::type_name::<Q>();
                let hash = calculate_hash(&query);

                if let Some(senders) = requests.get_mut(&hash) {
                    tracing::info!(key = hash, query = query_type, "coalescing a query");
                    senders.push(tx);
                } else {
                    requests.insert(hash, vec![tx]);
                    let (response_transmitter, response_receiver) = oneshot::channel();

                    let query_runner = query_runner.clone();
                    tokio::spawn(async move {
                        match query_runner.send(
                                QueryRunnerMessage {
                                    hash,
                                    query,
                                    response_transmitter
                                }
                            ).await {
                            Ok(_) => (),
                            Err(e) => {
                                let err = TrySendError::from(e);
                                tracing::error!(
                                    hash = hash,
                                    "error sending query to query runner: {:?}",
                                    err
                                );
                                // todo: propagate error to requestor
                            },
                        };
                    });

                    join_set.spawn(async move {
                        let res = response_receiver.await;
                        tracing::debug!(hash = hash, "joinset handle returned: {:#?}", res);

                        (hash, res)
                    });
                }
            },
            // this runs when the query is completed and needs be to dispatched to the requestors
            Some(join_handle) = join_set.join_next() => {
                tracing::debug!("join set recieved a result!");
                if let Ok((hash, result)) = join_handle {
                    if let Some(mut senders) = requests.remove(&hash) {
                        let res = result.unwrap();

                        let last_sender = senders.pop();

                        for sender in senders {
                            let _ = sender.send(res.clone());
                        }

                        if let Some(sender) = last_sender {
                            let _ = sender.send(res);
                        }
                    }
                }
            },
            else => {}
        }
    }
}

fn calculate_hash<Q: Hash>(query: &Q) -> u64 {
    let mut hasher = DefaultHasher::new();
    query.hash(&mut hasher);
    hasher.finish()
}

async fn run_query_runner<Q, Execute, Execution>(
    mut query_receiver: Receiver<QueryRunnerMessage<Q>>,
    mut execute: Execute,
) where
    Q: ReadQuery,
    Execute: FnMut(Q) -> Execution,
    Execution: std::future::Future<Output = Result<Q::Output, ScyllaxError>>,
{
    while let Some(QueryRunnerMessage {
        query,
        response_transmitter,
        hash,
    }) = query_receiver.recv().await
    {
        tracing::debug!("running query for hash: {hash}");
        let result = execute(query).await;
        let _ = response_transmitter.send(Arc::new(result));
    }
}

impl<T: QueryCollection> std::fmt::Display for Executor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.session)
    }
}

#[cfg(test)]
mod coalescing_tests {
    use super::*;
    use async_trait::async_trait;
    use std::{
        collections::HashSet,
        future::Future,
        hash::{Hash, Hasher},
        time::Duration,
    };

    #[derive(Debug, Hash, scylla::SerializeRow)]
    struct TestQuery {
        identity: i64,
    }

    impl Query for TestQuery {
        fn query() -> String {
            "test".to_owned()
        }
    }

    #[async_trait]
    impl ReadQuery for TestQuery {
        type Output = Option<u64>;

        async fn parse_response(_: QueryResult) -> Result<Self::Output, ScyllaxError> {
            unreachable!("database-free coalescing tests never parse a driver response")
        }
    }

    #[derive(Debug, Hash, scylla::SerializeRow)]
    struct OtherQuery {
        identity: i64,
    }

    impl Query for OtherQuery {
        fn query() -> String {
            "other test".to_owned()
        }
    }

    #[async_trait]
    impl ReadQuery for OtherQuery {
        type Output = Option<u64>;

        async fn parse_response(_: QueryResult) -> Result<Self::Output, ScyllaxError> {
            unreachable!("database-free coalescing tests never parse a driver response")
        }
    }

    #[derive(Debug, scylla::SerializeRow)]
    struct CollisionQuery {
        identity: i64,
    }

    impl Hash for CollisionQuery {
        fn hash<H: Hasher>(&self, state: &mut H) {
            0_i64.hash(state);
        }
    }

    impl Query for CollisionQuery {
        fn query() -> String {
            "collision test".to_owned()
        }
    }

    #[async_trait]
    impl ReadQuery for CollisionQuery {
        type Output = Option<u64>;

        async fn parse_response(_: QueryResult) -> Result<Self::Output, ScyllaxError> {
            unreachable!("database-free coalescing tests never parse a driver response")
        }
    }

    #[derive(Debug, scylla::SerializeRow)]
    struct CreatedBeforeQuery {
        created_before: i64,
        rowlimit: i32,
    }

    impl Hash for CreatedBeforeQuery {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.created_before.hash(state);
        }
    }

    impl Query for CreatedBeforeQuery {
        fn query() -> String {
            "created before test".to_owned()
        }
    }

    #[async_trait]
    impl ReadQuery for CreatedBeforeQuery {
        type Output = Option<u64>;

        async fn parse_response(_: QueryResult) -> Result<Self::Output, ScyllaxError> {
            unreachable!("database-free coalescing tests never parse a driver response")
        }
    }

    async fn within<F: Future>(future: F) -> F::Output {
        tokio::time::timeout(Duration::from_secs(1), future)
            .await
            .expect("coalescing synchronization timed out")
    }

    async fn submit<Q: ReadQuery>(
        requests: &Sender<ShardMessage<Q>>,
        query: Q,
    ) -> oneshot::Receiver<ReadQueryResult<Q>> {
        let (response_tx, response_rx) = oneshot::channel();
        within(requests.send((query, response_tx))).await.unwrap();
        response_rx
    }

    fn outcome<Q: ReadQuery>(value: Q::Output) -> ReadQueryResult<Q> {
        Arc::new(Ok(value))
    }

    #[tokio::test]
    async fn ten_identical_requests_admit_one_leader_and_share_its_outcome() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(16);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(16);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let mut responses = Vec::new();
        for _ in 0..10 {
            responses.push(submit(&request_tx, TestQuery { identity: 7 }).await);
        }

        let leader = within(runner_rx.recv()).await.unwrap();
        leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(7)))
            .unwrap();

        for response in responses {
            let outcome = within(response).await.unwrap();
            assert_eq!(outcome.as_ref().as_ref().unwrap(), &Some(7));
        }
        assert!(runner_rx.try_recv().is_err());

        task.abort();
    }

    #[tokio::test]
    async fn distinct_full_identities_admit_distinct_leaders() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let first = submit(&request_tx, TestQuery { identity: 1 }).await;
        let second = submit(&request_tx, TestQuery { identity: 2 }).await;
        let mut identities = HashSet::new();
        let leader = within(runner_rx.recv()).await.unwrap();
        identities.insert(leader.query.identity);
        leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(1)))
            .unwrap();
        let leader = within(runner_rx.recv()).await.unwrap();
        identities.insert(leader.query.identity);
        leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(2)))
            .unwrap();

        assert_eq!(identities, HashSet::from([1, 2]));
        within(first).await.unwrap();
        within(second).await.unwrap();
        task.abort();
    }

    #[tokio::test]
    async fn followers_before_and_during_leader_completion_resolve() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(8);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(8);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let first = submit(&request_tx, TestQuery { identity: 7 }).await;
        let before = submit(&request_tx, TestQuery { identity: 7 }).await;
        let leader = within(runner_rx.recv()).await.unwrap();
        let during = submit(&request_tx, TestQuery { identity: 7 }).await;
        let marker = submit(&request_tx, TestQuery { identity: 99 }).await;
        let marker_leader = within(runner_rx.recv()).await.unwrap();
        assert_eq!(marker_leader.query.identity, 99);

        leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(7)))
            .unwrap();
        marker_leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(99)))
            .unwrap();

        for response in [first, before, during] {
            let result = within(response).await.unwrap();
            assert_eq!(result.as_ref().as_ref().unwrap(), &Some(7));
        }
        within(marker).await.unwrap();
        task.abort();
    }

    #[tokio::test]
    async fn cancelling_one_follower_does_not_poison_the_rest() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(8);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(8);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let leader_response = submit(&request_tx, TestQuery { identity: 7 }).await;
        let cancelled = submit(&request_tx, TestQuery { identity: 7 }).await;
        drop(cancelled);
        let remaining = submit(&request_tx, TestQuery { identity: 7 }).await;
        let marker = submit(&request_tx, TestQuery { identity: 99 }).await;

        let mut leader = within(runner_rx.recv()).await.unwrap();
        let mut marker_leader = within(runner_rx.recv()).await.unwrap();
        if leader.query.identity == 99 {
            std::mem::swap(&mut leader, &mut marker_leader);
        }
        assert_eq!(leader.query.identity, 7);
        assert_eq!(marker_leader.query.identity, 99);
        leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(7)))
            .unwrap();
        marker_leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(99)))
            .unwrap();

        for response in [leader_response, remaining] {
            let result = within(response).await.unwrap();
            assert_eq!(result.as_ref().as_ref().unwrap(), &Some(7));
        }
        within(marker).await.unwrap();
        task.abort();
    }

    #[tokio::test]
    async fn empty_optional_result_fans_out_as_success() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let first = submit(&request_tx, TestQuery { identity: 7 }).await;
        let follower = submit(&request_tx, TestQuery { identity: 7 }).await;
        let leader = within(runner_rx.recv()).await.unwrap();
        leader
            .response_transmitter
            .send(outcome::<TestQuery>(None))
            .unwrap();

        for response in [first, follower] {
            let result = within(response).await.unwrap();
            assert_eq!(result.as_ref().as_ref().unwrap(), &None);
        }
        task.abort();
    }

    #[tokio::test]
    async fn coalescing_is_scoped_by_query_type() {
        let (first_tx, first_rx) = tokio::sync::mpsc::channel(2);
        let (first_runner_tx, mut first_runner_rx) = tokio::sync::mpsc::channel(2);
        let first_task = tokio::spawn(run_read_task::<TestQuery>(first_rx, first_runner_tx));
        let (second_tx, second_rx) = tokio::sync::mpsc::channel(2);
        let (second_runner_tx, mut second_runner_rx) = tokio::sync::mpsc::channel(2);
        let second_task = tokio::spawn(run_read_task::<OtherQuery>(second_rx, second_runner_tx));

        let first = submit(&first_tx, TestQuery { identity: 7 }).await;
        let second = submit(&second_tx, OtherQuery { identity: 7 }).await;
        let first_leader = within(first_runner_rx.recv()).await.unwrap();
        let second_leader = within(second_runner_rx.recv()).await.unwrap();
        first_leader
            .response_transmitter
            .send(outcome::<TestQuery>(Some(1)))
            .unwrap();
        second_leader
            .response_transmitter
            .send(outcome::<OtherQuery>(Some(2)))
            .unwrap();

        assert_eq!(
            within(first).await.unwrap().as_ref().as_ref().unwrap(),
            &Some(1)
        );
        assert_eq!(
            within(second).await.unwrap().as_ref().as_ref().unwrap(),
            &Some(2)
        );
        first_task.abort();
        second_task.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: unequal keys forced to the same hash must remain isolated"]
    async fn colliding_unequal_keys_remain_isolated() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(run_read_task::<CollisionQuery>(request_rx, runner_tx));

        let _first = submit(&request_tx, CollisionQuery { identity: 1 }).await;
        let _second = submit(&request_tx, CollisionQuery { identity: 2 }).await;
        let _first_leader = within(runner_rx.recv()).await.unwrap();
        let _second_leader = within(runner_rx.recv()).await.unwrap();
        task.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: created-before queries with different limits must remain isolated"]
    async fn created_before_queries_with_different_limits_remain_isolated() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(run_read_task::<CreatedBeforeQuery>(request_rx, runner_tx));

        let _first = submit(
            &request_tx,
            CreatedBeforeQuery {
                created_before: 7,
                rowlimit: 10,
            },
        )
        .await;
        let _second = submit(
            &request_tx,
            CreatedBeforeQuery {
                created_before: 7,
                rowlimit: 20,
            },
        )
        .await;
        let _first_leader = within(runner_rx.recv()).await.unwrap();
        let _second_leader = within(runner_rx.recv()).await.unwrap();
        task.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: distinct identities must execute concurrently"]
    async fn distinct_identities_execute_concurrently() {
        let (runner_tx, runner_rx) = tokio::sync::mpsc::channel(4);
        let first_release = Arc::new(tokio::sync::Notify::new());
        let second_started = Arc::new(tokio::sync::Notify::new());
        let runner = tokio::spawn(run_query_runner(runner_rx, {
            let first_release = first_release.clone();
            let second_started = second_started.clone();
            move |query: TestQuery| {
                let first_release = first_release.clone();
                let second_started = second_started.clone();
                async move {
                    if query.identity == 1 {
                        first_release.notified().await;
                    } else {
                        second_started.notify_one();
                    }
                    Ok(Some(query.identity as u64))
                }
            }
        }));

        for identity in [1, 2] {
            let (response_transmitter, _response_receiver) = oneshot::channel();
            runner_tx
                .send(QueryRunnerMessage {
                    hash: identity as u64,
                    query: TestQuery { identity },
                    response_transmitter,
                })
                .await
                .unwrap();
        }
        within(second_started.notified()).await;
        first_release.notify_one();
        runner.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: runner-send failure must resolve all followers"]
    async fn runner_send_failure_resolves_all_followers() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, runner_rx) = tokio::sync::mpsc::channel(1);
        drop(runner_rx);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let first = submit(&request_tx, TestQuery { identity: 7 }).await;
        let follower = submit(&request_tx, TestQuery { identity: 7 }).await;
        assert!(within(first).await.is_ok());
        assert!(within(follower).await.is_ok());
        task.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: runner task panic must resolve all followers"]
    async fn runner_task_panic_resolves_all_followers() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(4);
        let (runner_tx, mut runner_rx) = tokio::sync::mpsc::channel(4);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));

        let first = submit(&request_tx, TestQuery { identity: 7 }).await;
        let follower = submit(&request_tx, TestQuery { identity: 7 }).await;
        let leader = within(runner_rx.recv()).await.unwrap();
        let panic = tokio::spawn(async move {
            drop(leader);
            panic!("simulated runner panic");
        });
        assert!(panic.await.is_err());
        assert!(within(first).await.is_ok());
        assert!(within(follower).await.is_ok());
        task.abort();
    }

    #[tokio::test]
    #[ignore = "plan 003: dropping the executor must terminate tasks without busy-spin"]
    async fn dropping_executor_terminates_tasks_without_busy_spin() {
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(1);
        let (runner_tx, _runner_rx) = tokio::sync::mpsc::channel(1);
        let task = tokio::spawn(run_read_task::<TestQuery>(request_rx, runner_tx));
        drop(request_tx);

        within(task).await.unwrap();
    }
}
