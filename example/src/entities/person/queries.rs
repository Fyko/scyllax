use scyllax::PreparedStatement;
#[allow(non_snake_case)]
use scyllax::{delete_query, prelude::*};
use uuid::Uuid;

prepare_queries!(
    PersonEntityQueries,
    [
        GetPersonById,
        GetPeopleByIds,
        GetPersonByEmail,
        // DeletePersonById
    ]
);

/// Get a [`super::model::PersonEntity`] by its [`uuid::Uuid`]
// #[derive(scylla::ValueList, std::fmt::Debug, std::clone::Clone, PartialEq, Hash)]
#[select_query(
    query = "select * from person where id = ? limit 1",
    entity_type = "super::model::PersonEntity"
)]
pub struct GetPersonById {
    /// The [`uuid::Uuid`] of the [`super::model::PersonEntity`] to get
    pub id: Uuid,
}

// impl scyllax::GenericQuery<PersonEntity> for GetPersonById {
//     fn query() -> String {
//         "select * from person where id = ? limit 1"
//             .replace("*", &PersonEntity::keys().join(", "))
//     }
// }

// #[scyllax::async_trait]
// impl scyllax::SelectQuery<PersonEntity, PersonEntity, PersonQueries> for GetPersonById {
//     async fn execute(self, db: &scyllax::Executor<PersonQueries>) -> anyhow::Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
//         let statement = db.queries.get::<GetPersonById>();
//         tracing::debug!{
//             target = "GetPersonById",
//             "executing select"
//         };

//         db.session.execute(statement, self).await
//     }

//     async fn parse_response(res: scylla::QueryResult) -> Result<GetPersonById, scyllax::ScyllaxError> {
//         todo!()
//     }
// }

// pub struct PersonQueries {
//     GetPersonById: PreparedStatement,
// }

// #[scyllax::async_trait]
// #[doc = "A collection of prepared statements."]
// impl scyllax::Queries for PersonQueries {
//     async fn new(session: &scylla::Session) -> Result<Self, scyllax::ScyllaxError> {
//         Ok(Self {
//             GetPersonById: session.prepare(GetPersonById::query()).await?,
//         })
//     }

//     #[doc = "Get a prepared statement."]
//     fn get<T>(&self) -> &scylla::statement::prepared_statement::PreparedStatement
//     where
//         Self: scyllax::GetPreparedStatement<T>,
//     {
//         <Self as scyllax::GetPreparedStatement<T>>::get_prepared_statement(self)
//     }
// }

// impl scyllax::GetPreparedStatement<GetPersonById> for PersonQueries {
//     #[doc = "Get a prepared statement."]
//     fn get_prepared_statement(&self) -> &scylla::statement::prepared_statement::PreparedStatement {
//         &self.GetPersonById
//     }
// }

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

// fn test() {
//     use scylla::batch::*;

//     let batch = Batch::new_with_statements(
//         BatchType::Logged,
//         vec![
//             BatchStatement::Query(GenericQuery::<GetPersonById>::query()),
//             BatchStatement::Query(GenericQuery::<GetPeopleByIds>::query()),
//             BatchStatement::Query(GenericQuery::<GetPersonByEmail>::query()),
//             BatchStatement::Query(GenericQuery::<DeletePersonById>::query()),
//         ]
//     );
// }

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_person_by_id() {
        let _query = GetPersonById { id: v1_uuid() };

        assert_eq!(
            GetPersonById::query(),
            r#"select id, email, age, data, "createdAt" from person where id = ? limit 1"#
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
            r#"select id, email, age, data, "createdAt" from person where id in ? limit ?"#
        );
    }

    #[test]
    fn test_get_person_by_email() {
        let _query = GetPersonByEmail {
            email: "foo@scyllax.com".to_string(),
        };

        assert_eq!(
            GetPersonByEmail::query(),
            r#"select id, email, age, data, "createdAt" from person_by_email where email = ? limit 1"#
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
