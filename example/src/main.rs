//! Example
use entities::person::{
    model::UpsertPerson,
    queries::{GetPeopleByIds, GetPersonByEmail, GetPersonById},
};
use scyllax::prelude::*;
use scyllax::{executor::create_session, util::v1_uuid};
use tracing_subscriber::prelude::*;
use uuid::Uuid;

pub mod entities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let known_nodes = std::env::var("SCYLLA_NODES").unwrap_or_else(|_| String::new());
    let known_nodes = known_nodes.split(',').collect::<Vec<_>>();
    let default_keyspace = std::env::var("SCYLLA_DEFAULT_KEYSPACE").ok();

    let session = create_session(known_nodes, default_keyspace).await?;
    let executor = Executor::with_session(session);

    let query = GetPersonByEmail {
        email: "foo11@scyllax.local".to_string(),
    };
    let res_one = executor
        .execute_select(query)
        .await?
        .expect("person not found");
    tracing::debug!("query 1: {:?}", res_one);

    let query = GetPersonById { id: res_one.id };
    let res_two = executor
        .execute_select(query)
        .await?
        .expect("person not found");
    tracing::debug!("query 2: {:?}", res_two);
    assert_eq!(res_one, res_two);

    let ids = vec![
        "e01e84d6-414c-11ee-be56-0242ac120002",
        "e01e880a-414c-11ee-be56-0242ac120002",
    ]
    .iter()
    .map(|s| Uuid::parse_str(s).unwrap())
    .collect::<Vec<_>>();
    let query = GetPeopleByIds {
        limit: ids.len() as i32,
        ids,
    };
    let res = executor.execute_select(query).await?;
    tracing::debug!("query 3: {:?}", res);

    let query = UpsertPerson {
        id: v1_uuid(),
        email: MaybeUnset::Set("foo21@scyllax.local".to_string()),
        age: MaybeUnset::Set(Some(21)),
        created_at: MaybeUnset::Unset,
    };
    let res = executor.execute_upsert(query).await?;
    tracing::debug!("query 4: {:?}", res);

    Ok(())
}
