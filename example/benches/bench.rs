//! benches
use example::entities::person::{self, queries::PersonQueries};
use scyllax::prelude::{create_session, Executor};
use std::sync::Arc;
use tracing_subscriber::prelude::*;

async fn test_select(executor: Arc<Executor<PersonQueries>>) {
    let query = person::queries::GetPersonByEmail {
        email: "foo1@scyllax.local".to_string(),
    };

    let _ = executor
        .execute_read(query)
        .await
        .expect("person not found");
}

const RUNS: usize = 100_000;

#[tokio::main]
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
    for _ in 0..RUNS {
        test_select(executor.clone()).await;
    }
    let end = std::time::Instant::now();

    println!("elapsed: {:#?}", end - start);
    println!("per run: {:?}", (end - start) / RUNS as u32);

    Ok(())
}
