#![allow(dead_code)]

use std::{str::FromStr, sync::Arc};

use scylla::{CachingSession, FromRow, SessionBuilder, ValueList};
use scyllax::{executor::Executor, select_query, Entity, EntityExt, SelectQuery};
use tracing_subscriber::prelude::*;
use uuid::Uuid;

#[derive(Debug, Entity, ValueList, FromRow, Clone)]
struct Person {
    id: Uuid,
    email: String,
    #[rename("createdAt")]
    created_at: i64,
}

#[select_query(
    query = "select * from person where id = ? limit 1",
    entity_type = "Person"
)]
struct GetPersonById {
    id: Uuid,
}

async fn load(db: &mut Executor) -> anyhow::Result<()> {
    tracing::info!("loading queries");
    let _ = GetPersonById::prepare(db).await;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let session = create_db().await?;
    let mut executor = Executor::with_session(session);
    load(&mut executor).await?;

    let executor = Arc::new(executor);
    let query = GetPersonById {
        id: Uuid::from_str("e01ea1d2-414c-11ee-be56-0242ac120002")?,
    };
    let person = executor.execute_select(query).await?;

    println!("Person: {:#?}", person);

    Ok(())
}

pub async fn create_db() -> anyhow::Result<CachingSession> {
    let session = CachingSession::from(
        SessionBuilder::new()
            .known_node("127.0.0.1:9042")
            .build()
            .await?,
        1_000,
    );

    session.get_session().use_keyspace("scyllax", false).await?;

    Ok(session)
}
