use entities::person::queries::{GetPersonByEmail, GetPersonById};
use scyllax::executor::create_session;
use scyllax::prelude::*;
use tracing_subscriber::prelude::*;

pub mod entities;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let known_nodes = std::env::var("SCYLLA_NODES").unwrap_or_else(|_| String::new());
    let known_nodes = known_nodes.split(",").collect::<Vec<_>>();
    let default_keyspace = std::env::var("SCYLLA_DEFAULT_KEYSPACE").ok();
    let session = create_session(known_nodes, default_keyspace).await?;
    let exectuor = Executor::with_session(session);

    // TODO: run init

    let by_email = GetPersonByEmail {
        email: "foo11@scyllax.local".to_string(),
    };
    let res_one = exectuor
        .execute_select(by_email)
        .await?
        .expect("person not found");
    tracing::debug!("query 1: {:?}", res_one);

    let by_id = GetPersonById { id: res_one.id };
    let res_two = exectuor
        .execute_select(by_id)
        .await?
        .expect("person not found");
    tracing::debug!("query 2: {:?}", res_two);

    assert_eq!(res_one, res_two);

    let test = exectuor.session.execute(r##"insert into person(id, email, "createdAt") values (b4ee3e46-46ce-11ee-be56-0242ac120002, 'foo21@scyllax.local', toUnixTimestamp(now()));"##, ())
		.await?.rows();
    tracing::debug!("test: {:?}", test);

    Ok(())
}
