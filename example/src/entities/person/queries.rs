use scyllax::prelude::*;
use uuid::Uuid;

/// Load all queries for this entity
#[tracing::instrument(skip(db))]
pub async fn load(db: &mut Executor) -> anyhow::Result<()> {
    let _ = GetPersonById::prepare(db).await;

    Ok(())
}

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[select_query(
    query = "select * from person where id = ? limit 1",
    entity_type = "super::model::PersonEntity"
)]
pub struct GetPersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

/// Get many [`super::model::PersonEntity`] by many [`uuid::Uuid`]
#[select_query(
    query = "select * from person where id in ? limit ?",
    entity_type = "Vec<super::model::PersonEntity>"
)]
pub struct GetPeopleByIds {
    /// The [`uuid::Uuid`]s of the [`super::model::PersonEntity`]s to get
    pub ids: Vec<Uuid>,
    /// The maximum number of [`super::model::PersonEntity`]s to get
    pub limit: i32,
}

/// Get a [`super::model::PersonEntity`] by its email address
#[select_query(
    query = "select * from person_by_email where email = ? limit 1",
    entity_type = "super::model::PersonEntity"
)]
pub struct GetPersonByEmail {
    /// The email address of the [`super::model::PersonEntity`] to get
    pub email: String,
}
