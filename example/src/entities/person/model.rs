use scylla::ValueList;
use scyllax::prelude::*;

#[upsert_query(table = "person", name = UpsertPerson)]
#[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
pub struct PersonEntity {
    #[pk]
    pub id: uuid::Uuid,
    pub email: String,
    pub age: Option<i32>,
    #[rename("createdAt")]
    pub created_at: i64,
}

// struct UpsertPerson {
//     pub id: uuid::Uuid,
//     pub email: MaybeUnset<String>,
//     pub age: MaybeUnset<Option<i32>>,
//     pub first_name: MaybeUnset<String>,
// }

// TODO: macroify
// TODO: use insert if every field is a PK
// #[scyllax::async_trait]
// impl UpsertQuery<PersonEntity> for UpsertPerson {
//     fn query(
//         &self,
//     ) -> Result<(String, scylla::frame::value::SerializedValues), BuildUpsertQueryError> {
//         let mut query = String::from("update person set ");
//         let mut variables = scylla::frame::value::SerializedValues::new();

//         if let MaybeUnset::Set(first_name) = &self.first_name {
//             query.push_str(&format!(r##"first_name = ?, "##));

//             match variables.add_value(first_name) {
//                 Ok(_) => (),
//                 Err(SerializeValuesError::TooManyValues) => {
//                     return Err(BuildUpsertQueryError::TooManyValues {
//                         field: "first_name".to_string(),
//                     })
//                 }
//                 Err(SerializeValuesError::MixingNamedAndNotNamedValues) => {
//                     return Err(BuildUpsertQueryError::MixingNamedAndNotNamedValues)
//                 }
//                 Err(SerializeValuesError::ValueTooBig(_)) => {
//                     return Err(BuildUpsertQueryError::ValueTooBig {
//                         field: "first_name".to_string(),
//                     })
//                 }
//                 Err(SerializeValuesError::ParseError) => {
//                     return Err(BuildUpsertQueryError::ParseError {
//                         field: "first_name".to_string(),
//                     })
//                 }
//             }
//         }

//         if let MaybeUnset::Set(email) = &self.email {
//             query.push_str(r##"email = ?, "##);
//             match variables.add_value(email) {
//                 Ok(_) => (),
//                 Err(SerializeValuesError::TooManyValues) => {
//                     return Err(BuildUpsertQueryError::TooManyValues {
//                         field: "email".to_string(),
//                     })
//                 }
//                 Err(SerializeValuesError::MixingNamedAndNotNamedValues) => {
//                     return Err(BuildUpsertQueryError::MixingNamedAndNotNamedValues)
//                 }
//                 Err(SerializeValuesError::ValueTooBig(_)) => {
//                     return Err(BuildUpsertQueryError::ValueTooBig {
//                         field: "email".to_string(),
//                     })
//                 }
//                 Err(SerializeValuesError::ParseError) => {
//                     return Err(BuildUpsertQueryError::ParseError {
//                         field: "email".to_string(),
//                     })
//                 }
//             }
//         }

//         if let MaybeUnset::Set(age) = &self.age {
//             // age is also optional, so we have to unwrap it
//             if let Some(age) = age {
//                 query.push_str("age = ?, ");
//                 match variables.add_value(age) {
//                     Ok(_) => (),
//                     Err(SerializeValuesError::TooManyValues) => {
//                         return Err(BuildUpsertQueryError::TooManyValues {
//                             field: "age".to_string(),
//                         })
//                     }
//                     Err(SerializeValuesError::MixingNamedAndNotNamedValues) => {
//                         return Err(BuildUpsertQueryError::MixingNamedAndNotNamedValues)
//                     }
//                     Err(SerializeValuesError::ValueTooBig(_)) => {
//                         return Err(BuildUpsertQueryError::ValueTooBig {
//                             field: "age".to_string(),
//                         })
//                     }
//                     Err(SerializeValuesError::ParseError) => {
//                         return Err(BuildUpsertQueryError::ParseError {
//                             field: "age".to_string(),
//                         })
//                     }
//                 }
//             }
//         }

//         query.pop();
//         query.pop();
//         query.push_str(" where id = ?;");
//         match variables.add_value(&self.id) {
//             Ok(_) => (),
//             Err(SerializeValuesError::TooManyValues) => {
//                 return Err(BuildUpsertQueryError::TooManyValues {
//                     field: "id".to_string(),
//                 })
//             }
//             Err(SerializeValuesError::MixingNamedAndNotNamedValues) => {
//                 return Err(BuildUpsertQueryError::MixingNamedAndNotNamedValues)
//             }
//             Err(SerializeValuesError::ValueTooBig(_)) => {
//                 return Err(BuildUpsertQueryError::ValueTooBig {
//                     field: "id".to_string(),
//                 })
//             }
//             Err(SerializeValuesError::ParseError) => {
//                 return Err(BuildUpsertQueryError::ParseError {
//                     field: "id".to_string(),
//                 })
//             }
//         }

//         Ok((query, variables))
//     }

//     async fn execute(
//         self,
//         db: &scyllax::Executor,
//     ) -> anyhow::Result<scylla::QueryResult, ScyllaxError> {
//         let (query, values) = self.query()?;

//         db.session.execute(query, values).await.map_err(|e| e.into())
//     }
// }
