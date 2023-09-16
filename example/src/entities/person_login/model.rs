use scyllax::prelude::*;

/// Represents a person in the database
#[entity]
#[upsert_query(table = "person_login", name = UpsertPersonLogin)]
pub struct PersonLoginEntity {
    /// The id of the person
    #[pk]
    pub id: uuid::Uuid,
    /// The email address of the person
    #[pk]
    pub person_id: uuid::Uuid,
    /// The number of times the person has logged in
    #[counter]
    pub count: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pks() {
        assert_eq!(
            PersonLoginEntity::pks(),
            vec!["id".to_string(), "person_id".to_string()]
        );
    }

    #[test]
    fn test_keys() {
        assert_eq!(
            PersonLoginEntity::keys(),
            vec![
                "id".to_string(),
                "person_id".to_string(),
                "count".to_string()
            ]
        );
    }

    #[test]
    fn test_upsert() {
        let upsert = UpsertPersonLogin {
            id: v1_uuid(),
            person_id: v1_uuid(),
            count: 1.into(),
        };

        let (query, values) = upsert.query().expect("failed to parse into query");

        assert_eq!(
            query,
            r#"update person_login set count = count + :count where id = :id and person_id = :person_id;"#
        );

        let mut result_values = SerializedValues::new();
        result_values
            .add_named_value("count", &upsert.count)
            .expect("failed to add value");
        result_values
            .add_named_value("id", &upsert.id)
            .expect("failed to add value");
        result_values
            .add_named_value("person_id", &upsert.person_id)
            .expect("failed to add value");

        assert_eq!(values, result_values);
    }
}
