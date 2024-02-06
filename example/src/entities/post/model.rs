use scyllax::{json::Json, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LikeData {
    pub user_id: Uuid,
    pub created_at: i64,
}

/// Represents a post in the database
#[entity]
#[upsert_query(table = "post", name = UpsertPost)]
pub struct PostEntity {
    /// The id of the post
    #[entity(primary_key)]
    pub id: Uuid,
    /// The title of the post
    pub title: String,
    /// The likes on the post (dont store likes like this in production, this is just to test Json<T>)
    pub likes: Option<Json<Vec<LikeData>>>,
    /// The date the post was created
    pub created_at: i64,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_pks() {
        assert_eq!(PostEntity::pks(), vec![r#""id""#.to_string()]);
    }

    #[test]
    fn test_keys() {
        assert_eq!(
            PostEntity::keys(),
            vec![
                r#""id""#.to_string(),
                r#""title""#.to_string(),
                r#""data""#.to_string(),
                r#""created_at""#.to_string()
            ]
        );
    }
}
