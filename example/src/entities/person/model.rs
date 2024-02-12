use scylla::frame::value::{CqlTimestamp, CqlTimeuuid};
use scylla::serialize::value::SerializeCql;
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
#[upsert_query(table = "person", name = UpsertPersonWithTTL, ttl)]
pub struct PersonEntity {
    /// The id of the person
    #[entity(primary_key)]
    pub id: CqlTimeuuid,
    /// The email address of the person
    pub email: String,
    /// The age of the person
    pub age: Option<i32>,
    /// Other data from the person
    pub data: Option<PersonData>,
    /// The kind of person
    pub kind: PersonKind,
    /// The date the person was created
    #[entity(rename = "createdAt")]
    #[scylla(rename = "createdAt")]
    pub created_at: CqlTimestamp,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pks() {
        assert_eq!(PersonEntity::pks(), vec![r#""id""#.to_string()]);
    }

    #[test]
    fn test_keys() {
        assert_eq!(
            PersonEntity::keys(),
            vec![
                r#""id""#.to_string(),
                r#""email""#.to_string(),
                r#""age""#.to_string(),
                r#""data""#.to_string(),
                r#""kind""#.to_string(),
                r#""createdAt""#.to_string()
            ]
        );
    }
}
