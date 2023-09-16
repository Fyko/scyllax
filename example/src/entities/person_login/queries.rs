use scyllax::{delete_query, prelude::*};
use uuid::Uuid;

/// Get a [`super::model::PersonLoginEntity`] by its [`uuid::Uuid`]
#[select_query(
    query = "select * from person_login where id = ? limit 1",
    entity_type = "super::model::PersonLoginEntity"
)]
pub struct GetPersonLoginById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

/// Get a [`super::model::PersonLoginEntity`] by its [`uuid::Uuid`]
#[delete_query(
    query = "delete from person_login where id = ?",
    entity_type = "super::model::PersonLoginEntity"
)]
pub struct DeletePersonLoginById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonLoginEntity`] to get
    pub id: Uuid,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_person_by_id() {
        let _query = GetPersonLoginById { id: v1_uuid() };

        assert_eq!(
            GetPersonLoginById::query(),
            r#"select id, person_id, count from person_login where id = ? limit 1"#
        );
    }

    #[test]
    fn test_delete_person_by_id() {
        let _query = DeletePersonLoginById { id: v1_uuid() };

        assert_eq!(
            DeletePersonLoginById::query(),
            r#"delete from person_login where id = ?"#
        );
    }
}
