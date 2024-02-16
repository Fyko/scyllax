//! Example
use std::sync::Arc;

use example::entities::post::{
    model::{LikeData, UpsertPost},
    queries::{GetPostById, PostQueries},
};
use scylla::frame::value::{CqlTimestamp, CqlTimeuuid};
use scyllax::{executor::create_session, util::v1_uuid};
use scyllax::{json::Json, prelude::*};
use tracing_subscriber::prelude::*;
use uuid::Uuid;

#[inline]
fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

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
    let executor = Executor::<PostQueries>::new(Arc::new(session)).await?;

    let post_id = CqlTimeuuid::from(v1_uuid());
    let insert_body = UpsertPost {
        id: post_id,
        title: MaybeUnset::Set("Hello, World!".to_string()),
        likes: MaybeUnset::Set(Some(Json(vec![LikeData {
            user_id: Uuid::new_v4(),
            created_at: now(),
        }]))),
        created_at: MaybeUnset::Set(CqlTimestamp(now())),
    };
    eprintln!("inserting post: {insert_body:#?}");
    executor.execute_write(insert_body).await?;

    let post = executor.execute_read(GetPostById { id: post_id }).await?;
    println!("Post: {post:#?}");

    Ok(())
}
