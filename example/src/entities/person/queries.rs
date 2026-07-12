use super::model::{UpsertPerson, UpsertPersonWithTTL};
use scylla::{frame::value::CqlTimeuuid, SerializeRow};
use scyllax::prelude::*;
use value::CqlTimestamp;

create_query_collection!(
    PersonQueries,
    [
        GetPersonById,
        GetPeopleByIds,
        GetPersonByEmail,
        GetPeopleCreatedBefore
    ],
    [DeletePersonById, UpsertPerson, UpsertPersonWithTTL]
);

#[inline]
fn hash_cql_timestamp(timestamp: &CqlTimestamp) -> i64 {
    timestamp.0
}

#[derive(Debug, Clone, PartialEq, SerializeRow, ReadQuery)]
#[read_query(
    query = r#"select * from "person_by_createdAt" where "createdAt" <= :created_before limit :rowlimit"#,
    return_type = "Vec<super::model::PersonEntity>"
)]
pub struct GetPeopleCreatedBefore {
    #[read_query(coalesce_shard_key, hash_fn = "hash_cql_timestamp")]
    pub created_before: CqlTimestamp,
    pub rowlimit: i32,
}

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[derive(Debug, Clone, PartialEq, SerializeRow, ReadQuery)]
#[read_query(
    query = "select * from person where id = :id limit 1",
    return_type = "super::model::PersonEntity"
)]
pub struct GetPersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    #[read_query(coalesce_shard_key)]
    pub id: CqlTimeuuid,
}

/// Get many [`super::model::PersonEntity`] by many [`uuid::Uuid`]
#[derive(Debug, Clone, PartialEq, SerializeRow, ReadQuery)]
#[read_query(
    query = "select * from person where id in :ids limit :rowlimit",
    return_type = "Vec<super::model::PersonEntity>"
)]
pub struct GetPeopleByIds {
    /// The [`uuid::Uuid`]s of the [`super::model::PersonEntity`]s to get
    pub ids: Vec<CqlTimeuuid>,
    /// The maximum number of [`super::model::PersonEntity`]s to get
    pub rowlimit: i32,
}

/// Get a [`super::model::PersonEntity`] by its email address
#[derive(Debug, Clone, PartialEq, SerializeRow, ReadQuery)]
#[read_query(
    query = "select * from person_by_email where email = :email limit 1",
    return_type = "super::model::PersonEntity"
)]
pub struct GetPersonByEmail {
    /// The email address of the [`super::model::PersonEntity`] to get
    #[read_query(coalesce_shard_key)]
    pub email: String,
}

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
#[write_query(query = "delete from person where id = :id")]
pub struct DeletePersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: CqlTimeuuid,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    fn identity_hash(value: &impl Hash) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_get_person_by_id() {
        let _query = GetPersonById {
            id: CqlTimeuuid::from(v1_uuid()),
        };

        assert_eq!(
            GetPersonById::query(),
            r#"select "id", "email", "age", "data", "kind", "createdAt" from person where id = :id limit 1"#
        );
    }

    #[test]
    fn test_get_people_by_ids() {
        let _query = GetPeopleByIds {
            ids: vec![CqlTimeuuid::from(v1_uuid()), CqlTimeuuid::from(v1_uuid())],
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
        let _query = DeletePersonById {
            id: CqlTimeuuid::from(v1_uuid()),
        };

        assert_eq!(
            DeletePersonById::query(),
            r#"delete from person where id = :id"#
        );
    }

    #[test]
    fn default_identity_hash_includes_all_fields() {
        let ids = vec![CqlTimeuuid::from(v1_uuid())];
        let first = GetPeopleByIds {
            ids: ids.clone(),
            rowlimit: 10,
        };
        let second = GetPeopleByIds { ids, rowlimit: 20 };

        assert_ne!(identity_hash(&first), identity_hash(&second));
    }

    #[test]
    fn selected_created_before_shard_key_excludes_rowlimit() {
        let first = GetPeopleCreatedBefore {
            created_before: CqlTimestamp(7),
            rowlimit: 10,
        };
        let second = GetPeopleCreatedBefore {
            created_before: CqlTimestamp(7),
            rowlimit: 20,
        };

        assert_eq!(identity_hash(&first), identity_hash(&second));
    }

    #[test]
    #[ignore = "plan 003: created-before identity must include rowlimit"]
    fn created_before_with_different_limits_has_distinct_identity() {
        let first = GetPeopleCreatedBefore {
            created_before: CqlTimestamp(7),
            rowlimit: 10,
        };
        let second = GetPeopleCreatedBefore {
            created_before: CqlTimestamp(7),
            rowlimit: 20,
        };

        assert_ne!(identity_hash(&first), identity_hash(&second));
    }
}
