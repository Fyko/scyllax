use entities::person::{
    model::UpsertPerson,
    queries::{GetPersonByEmail, GetPersonById},
};
use scyllax::prelude::*;
use scyllax::{executor::create_session, util::v1_uuid};
use tracing_subscriber::prelude::*;

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

    let by_email = GetPersonByEmail {
        email: "foo11@scyllax.local".to_string(),
    };
    let res_one = executor
        .execute_select(by_email)
        .await?
        .expect("person not found");
    tracing::debug!("query 1: {:?}", res_one);

    let by_id = GetPersonById { id: res_one.id };
    let res_two = executor
        .execute_select(by_id)
        .await?
        .expect("person not found");
    tracing::debug!("query 2: {:?}", res_two);

    assert_eq!(res_one, res_two);

    let create = UpsertPerson {
        id: v1_uuid(),
        email: MaybeUnset::Set("foo21@scyllax.local".to_string()),
        age: MaybeUnset::Set(Some(21)),
        created_at: MaybeUnset::Unset,
    };
    let res_three = executor.execute_upsert(create).await?;
    tracing::debug!("query 3: {:?}", res_three);

    Ok(())
}
