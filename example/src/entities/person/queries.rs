use super::model::{UpsertPerson, UpsertPersonWithTTL};
use scyllax::prelude::*;
use uuid::Uuid;

create_query_collection!(
    PersonQueries,
    [
        GetPersonById,
        GetPeopleByIds,
        GetPersonByEmail,
        DeletePersonById,
        UpsertPerson,
        UpsertPersonWithTTL,
    ]
);

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[read_query(
    query = "select * from person where id = :id limit 1",
    return_type = "super::model::PersonEntity"
)]
pub struct GetPersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

/// Get many [`super::model::PersonEntity`] by many [`uuid::Uuid`]
#[read_query(
    query = "select * from person where id in :ids limit :rowlimit",
    return_type = "Vec<super::model::PersonEntity>"
)]
pub struct GetPeopleByIds {
    /// The [`uuid::Uuid`]s of the [`super::model::PersonEntity`]s to get
    pub ids: Vec<Uuid>,
    /// The maximum number of [`super::model::PersonEntity`]s to get
    pub rowlimit: i32,
}

/// Get a [`super::model::PersonEntity`] by its email address
#[read_query(
    query = "select * from person_by_email where email = :email limit 1",
    return_type = "super::model::PersonEntity"
)]
pub struct GetPersonByEmail {
    /// The email address of the [`super::model::PersonEntity`] to get
    pub email: String,
}

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[write_query(query = "delete from person where id = :id")]
pub struct DeletePersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_person_by_id() {
        let _query = GetPersonById { id: v1_uuid() };

        assert_eq!(
            GetPersonById::query(),
            r#"select "id", "email", "age", "data", "kind", "createdAt" from person where id = :id limit 1"#
        );
    }

    #[test]
    fn test_get_people_by_ids() {
        let _query = GetPeopleByIds {
            ids: vec![v1_uuid(), v1_uuid()],
            rowlimit: 10,
        };

        assert_eq!(
            GetPeopleByIds::query(),
            r#"select "id", "email", "age", "data", "kind", "createdAt" from person where id in :ids limit :rowlimit"#
        );
    }

    #[test]
    fn test_get_person_by_email() {
        let _query = GetPersonByEmail {
            email: "foo@scyllax.com".to_string(),
        };

        assert_eq!(
            GetPersonByEmail::query(),
            r#"select "id", "email", "age", "data", "kind", "createdAt" from person_by_email where email = :email limit 1"#
        );
    }

    #[test]
    fn test_delete_person_by_id() {
        let _query = DeletePersonById { id: v1_uuid() };

        assert_eq!(
            DeletePersonById::query(),
            r#"delete from person where id = :id"#
        );
    }
}
