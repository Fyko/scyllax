//! benches
use std::sync::Arc;

use example::entities::{
    person::{self, queries::PersonQueries},
    PersonEntity,
};
use scyllax::prelude::{create_session, Executor};
use tracing_subscriber::prelude::*;

async fn test_select(executor: Arc<Executor<PersonQueries>>) -> Option<PersonEntity> {
    let query = person::queries::GetPersonByEmail {
        email: "foo1@scyllax.local".to_string(),
    };

    executor
        .execute_read(query)
        .await
        .expect("person not found")
}

const RUNS: usize = 1000;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let known_nodes = std::env::var("SCYLLA_NODES").unwrap_or_else(|_| String::new());
    let known_nodes = known_nodes.split(',').collect::<Vec<_>>();
    let default_keyspace = std::env::var("SCYLLA_DEFAULT_KEYSPACE").ok();

    let session = create_session(known_nodes, default_keyspace).await?;
    let executor = Arc::new(Executor::<PersonQueries>::new(Arc::new(session)).await?);
    let start = std::time::Instant::now();

    let futures: Vec<_> = (0..RUNS)
        .map(|_| {
            let executor = executor.clone();
            tokio::spawn(test_select(executor))
        })
        .collect();
    let mut res = Vec::with_capacity(futures.len());
    for f in futures.into_iter() {
        res.push(f.await.unwrap());
    }

    let end = std::time::Instant::now();
    println!("elapsed: {:#?}", end - start);
    println!("per run: {:?}", (end - start) / RUNS as u32);

    Ok(())
}
