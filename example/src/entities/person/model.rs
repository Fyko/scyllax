use scyllax::prelude::*;

/// Represents data from a person
#[json_data]
pub struct PersonData {
    /// The stripe id of the person
    #[serde(rename = "stripeId")]
    pub stripe_id: Option<String>,
}

/// Represents the kind of person
#[int_enum]
pub enum PersonKind {
    /// The person is a staff member
    Staff = 0,
    /// The person is a parent
    Parent = 1,
    /// The person is a student
    Student = 2,
}

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
    /// Other data from the person
    pub data: Option<PersonData>,
    /// The kind of person
    pub kind: PersonKind,
    /// The date the person was created
    #[rename("createdAt")]
    pub created_at: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entities::person::model::UpsertPerson;
    use pretty_assertions::assert_eq;

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
                "data".to_string(),
                "\"createdAt\"".to_string()
            ]
        );
    }

    #[test]
    fn test_upsert() {
        let upsert = UpsertPerson {
            id: v1_uuid(),
            email: MaybeUnset::Set("foo21@scyllax.local".to_string()),
            age: MaybeUnset::Unset,
            kind: MaybeUnset::Set(PersonKind::Parent),
            data: MaybeUnset::Set(Some(PersonData {
                stripe_id: Some("stripe_id".to_string()),
            })),
            created_at: MaybeUnset::Unset,
        };

        let (query, values) = upsert.query().expect("failed to parse into query");

        assert_eq!(
            query,
            r#"update person set email = :email, age = :age, data = :data, "createdAt" = :created_at where id = :id;"#
        );

        let mut result_values = SerializedValues::new();
        result_values
            .add_named_value("email", &upsert.email)
            .expect("failed to add value");
        result_values
            .add_named_value("age", &upsert.age)
            .expect("failed to add value");
        result_values
            .add_named_value("data", &upsert.data)
            .expect("failed to add value");
        result_values
            .add_named_value("created_at", &upsert.created_at)
            .expect("failed to add value");
        result_values
            .add_named_value("id", &upsert.id)
            .expect("failed to add value");

        assert_eq!(values, result_values);
    }
}
