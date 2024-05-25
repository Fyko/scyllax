//! Example
use std::{str::FromStr, sync::Arc};

use example::entities::{
    person::{
        model::{PersonData, PersonKind, UpsertPerson, UpsertPersonWithTTL},
        queries::{
            DeletePersonById, GetPeopleByIds, GetPeopleCreatedBefore, GetPersonByEmail,
            GetPersonById, PersonQueries,
        },
    },
    PersonEntity,
};
use scylla::frame::value::CqlTimeuuid;
use scyllax::prelude::*;
use scyllax::{executor::create_session, util::v1_uuid};
use time::{Duration, OffsetDateTime};
use tracing_subscriber::prelude::*;
use value::CqlTimestamp;

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
    let executor = Executor::<PersonQueries>::new(Arc::new(session)).await?;

    let by_email_res = by_email(&executor, "foo1@scyllax.local".to_string()).await?;
    let by_id_res = by_id(&executor, by_email_res.id).await?;
    assert_eq!(by_email_res, by_id_res);

    let ids = [
        "e01e84d6-414c-11ee-be56-0242ac120002",
        "e01e880a-414c-11ee-be56-0242ac120002",
    ]
    .iter()
    .map(|s| CqlTimeuuid::from_str(s).unwrap())
    .collect::<Vec<_>>();
    by_ids(&executor, ids).await?;

    let upsert_id = CqlTimeuuid::from(v1_uuid());
    let query = UpsertPerson {
        id: upsert_id,
        email: "foo21@scyllax.local".to_string().into(),
        age: MaybeUnset::Set(Some(21)),
        data: MaybeUnset::Set(Some(PersonData {
            stripe_id: Some("stripe_id".to_string()),
        })),
        kind: MaybeUnset::Set(PersonKind::Parent),
        created_at: MaybeUnset::Unset,
    };
    let res = executor.execute_write(query).await?;
    tracing::info!("UpsertPerson returned: {:?}", res);

    let delete = DeletePersonById { id: upsert_id };
    let res = executor.execute_write(delete).await?;
    tracing::info!("DeletePersonById returned: {:?}", res);

    let upsert_ttl_id = CqlTimeuuid::from(v1_uuid());
    let query = UpsertPersonWithTTL {
        id: upsert_ttl_id,
        email: "foo42@scyllax.local".to_string().into(),
        age: MaybeUnset::Set(Some(42)),
        data: MaybeUnset::Set(Some(PersonData {
            stripe_id: Some("stripe_id".to_string()),
        })),
        kind: MaybeUnset::Set(PersonKind::Parent),
        created_at: MaybeUnset::Unset,

        // 5 minutes
        set_ttl: 300,
    };
    let res = executor.execute_write(query).await?;
    tracing::info!("UpsertPersonWithTTL returned: {:?}", res);

    let old_user_id = v1_uuid();
    let one_year_ago = OffsetDateTime::now_utc() - Duration::days(365);
    let query = UpsertPerson {
        id: CqlTimeuuid::from(old_user_id),
        email: MaybeUnset::Set("foo55@scyllax.local".to_string()),
        age: MaybeUnset::Set(Some(55)),
        data: MaybeUnset::Unset,
        kind: MaybeUnset::Set(PersonKind::Staff),
        created_at: MaybeUnset::Set(CqlTimestamp::from(one_year_ago)),
    };
    executor.execute_write(query).await?;

    let get_old = executor
        .execute_read(GetPeopleCreatedBefore {
            created_before: CqlTimestamp::from(OffsetDateTime::now_utc() - Duration::weeks(12)),
            rowlimit: 10,
        })
        .await?;
    assert!(
        get_old
            .iter()
            .any(|p| p.id == CqlTimeuuid::from(old_user_id)),
        "Old user not found"
    );

    executor
        .execute_write(DeletePersonById {
            id: CqlTimeuuid::from(old_user_id),
        })
        .await?;

    Ok(())
}

async fn by_email(
    executor: &Executor<PersonQueries>,
    email: String,
) -> anyhow::Result<PersonEntity> {
    let res = executor
        .execute_read(GetPersonByEmail { email })
        .await?
        .expect("person not found");

    tracing::info!("GetPersonByEmail returned: {:?}", res);

    Ok(res)
}

async fn by_id(
    executor: &Executor<PersonQueries>,
    id: CqlTimeuuid,
) -> anyhow::Result<PersonEntity> {
    let res = executor
        .execute_read(GetPersonById { id })
        .await?
        .expect("person not found");

    tracing::info!("GetPersonById returned: {:?}", res);

    Ok(res)
}

async fn by_ids(
    executor: &Executor<PersonQueries>,
    ids: Vec<CqlTimeuuid>,
) -> anyhow::Result<Vec<PersonEntity>> {
    let res = executor
        .execute_read(GetPeopleByIds { ids, rowlimit: 10 })
        .await?;

    tracing::info!("GetPeopleByIds returned: {:?}", res);

    Ok(res)
}
