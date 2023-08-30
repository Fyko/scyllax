use scyllax::prelude::*;
use uuid::Uuid;

#[tracing::instrument(skip(db))]
pub async fn load(db: &mut Executor) -> anyhow::Result<()> {
    let _ = GetPersonById::prepare(db).await;

    Ok(())
}

#[select_query(
    query = "select * from person where id = ? limit 1",
    entity_type = "super::model::PersonEntity"
)]
pub struct GetPersonById {
    pub id: Uuid,
}

#[select_query(
    query = "select * from person_by_email where email = ? limit 1",
    entity_type = "super::model::PersonEntity"
)]
pub struct GetPersonByEmail {
    pub email: String,
}
