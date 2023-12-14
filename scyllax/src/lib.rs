//! # Scyllax (sɪl-æks)
//! A SQLx and Discord inspired query system for ScyllaDB
//!
//! ## Example
//! ### 1. Model definition
//! Before you can write any queries, you have to define a model.
//! ```rust,ignore
//! #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
//! pub struct PersonEntity {
//!     #[pk]
//!     pub id: uuid::Uuid,
//!     pub email: String,
//!     pub created_at: i64,
//! }
//! ```
//! ### 2. Select queries
//! With the [`select_query`] attribute, it's easy to define select queries.
//! ```rust,ignore
//! #[read_query(
//!     query = "select * from person where id = ? limit 1",
//!     entity_type = "PersonEntity"
//! )]
//! pub struct GetPersonById {
//!     pub id: Uuid,
//! }
//! ```
//! ### 3. Upsert queries
//! With the [`upsert_query`] attribute, it's easy to define upsert queries.
//! ```rust,ignore
//! #[upsert_query(table = "person", name = UpsertPerson)]
//! #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
//! pub struct PersonEntity {
//!     #[pk]
//!     pub id: uuid::Uuid,
//!     pub email: String,
//!     pub created_at: i64,
//! }
//! ```
pub mod collection;
pub mod entity;
pub mod error;
pub mod executor;
#[cfg(feature = "json")]
pub mod json;
pub mod maybe_unset;
// mod playground;
pub mod prelude;
pub mod queries;
pub mod rows;
pub mod util;
