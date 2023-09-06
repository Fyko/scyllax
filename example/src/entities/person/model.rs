use scyllax::prelude::*;

/// Represents a person in the database
#[entity]
#[upsert_query(table = "person", name = UpsertPerson)]
pub struct PersonEntity {
    /// The id of the person
    #[pk]
    pub id: uuid::Uuid,
    /// The email address of the person
    pub email: String,
    /// The age of the person
    pub age: Option<i32>,
    /// The date the person was created
    #[rename("createdAt")]
    pub created_at: i64,
}

#[cfg(test)]
mod test {
    use super::PersonEntity;
    use crate::entities::person::model::UpsertPerson;
    use pretty_assertions::assert_eq;
    use scyllax::prelude::*;

    #[test]
    fn test_pks() {
        assert_eq!(PersonEntity::pks(), vec!["id".to_string()]);
    }

    #[test]
    fn test_keys() {
        assert_eq!(
            PersonEntity::keys(),
            vec![
                "id".to_string(),
                "email".to_string(),
                "age".to_string(),
                "\"createdAt\"".to_string()
            ]
        );
    }

    #[test]
    fn test_upsert_v1() {
        let upsert = UpsertPerson {
            id: v1_uuid(),
            email: MaybeUnset::Set("foo21@scyllax.local".to_string()),
            age: MaybeUnset::Set(Some(21)),
            created_at: MaybeUnset::Unset,
        };

        let (query, values) = upsert.query().expect("failed to parse into query");

        assert_eq!(
            query,
            r#"update "person" set "email" = ?, "age" = ? where "id" = ?;"#
        );

        let mut result_values = SerializedValues::new();
        result_values
            .add_value(&upsert.email)
            .expect("failed to add value");
        result_values
            .add_value(&upsert.age)
            .expect("failed to add value");
        result_values
            .add_value(&upsert.id)
            .expect("failed to add value");

        assert_eq!(values, result_values);
    }

    #[test]
    fn test_upsert_v2() {
        let upsert = UpsertPerson {
            id: v1_uuid(),
            email: MaybeUnset::Set("foo21@scyllax.local".to_string()),
            age: MaybeUnset::Unset,
            created_at: MaybeUnset::Unset,
        };

        let (query, values) = upsert.query().expect("failed to parse into query");

        assert_eq!(query, r#"update "person" set "email" = ? where "id" = ?;"#);

        let mut result_values = SerializedValues::new();
        result_values
            .add_value(&upsert.email)
            .expect("failed to add value");
        result_values
            .add_value(&upsert.id)
            .expect("failed to add value");

        assert_eq!(values, result_values);
    }
}
