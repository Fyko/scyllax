use scyllax::{delete_query, prelude::*};
use uuid::Uuid;

/// Load all queries for this entity
#[tracing::instrument(skip(db))]
pub async fn load(db: &mut Executor) -> anyhow::Result<()> {
    let _ = GetPersonById::prepare(db).await;
    let _ = GetPeopleByIds::prepare(db).await;
    let _ = GetPersonByEmail::prepare(db).await;
    let _ = DeletePersonById::prepare(db).await;

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

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[delete_query(
    query = "delete from person where id = ?",
    entity_type = "super::model::PersonEntity"
)]
pub struct DeletePersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

#[cfg(test)]
mod test {
    use super::*;
    use scyllax::prelude::*;

    #[test]
    fn test_get_person_by_id() {
        let _query = GetPersonById { id: v1_uuid() };

        assert_eq!(
            GetPersonById::query(),
            r#"select id, email, age, "createdAt" from person where id = ? limit 1"#
        );
    }

    #[test]
    fn test_get_people_by_ids() {
        let _query = GetPeopleByIds {
            ids: vec![v1_uuid(), v1_uuid()],
            limit: 10,
        };

        assert_eq!(
            GetPeopleByIds::query(),
            r#"select id, email, age, "createdAt" from person where id in ? limit ?"#
        );
    }

    #[test]
    fn test_get_person_by_email() {
        let _query = GetPersonByEmail {
            email: "foo@scyllax.com".to_string(),
        };

        assert_eq!(
            GetPersonByEmail::query(),
            r#"select id, email, age, "createdAt" from person_by_email where email = ? limit 1"#
        );
    }

    #[test]
    fn test_delete_person_by_id() {
        let _query = DeletePersonById { id: v1_uuid() };

        assert_eq!(
            DeletePersonById::query(),
            r#"delete from person where id = ?"#
        );
    }
}
