//! Scyllax macros. See the scyllax for more information.
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod entity;
mod r#enum;
mod json;
mod queries;

pub(crate) fn token_stream_with_error(mut tokens: TokenStream2, error: syn::Error) -> TokenStream2 {
    tokens.extend(error.into_compile_error());
    tokens
}

/// Apply this attribute to a struct to generate a select query.
/// ## Single result
/// ```rust,ignore
/// #[select_query(
///     query = "select * from person where id = ? limit 1",
///     entity_type = "PersonEntity"
/// )]
/// pub struct GetPersonById {
///     pub id: Uuid,
/// }
/// executor.execute_select(GetPersonById { id }).await?;
/// // -> Option<PersonEntity>
/// ```
/// ## Multiple results
/// ```rust,ignore
/// #[select_query(
///     query = "select * from person where id in ? limit ?",
///     entity_type = "Vec<PersonEntity>"
/// )]
/// pub struct GetPeopleByIds {
///     pub ids: Vec<Uuid>,
///     pub limit: i32,
/// }
/// executor.execute_select(GetPeopleByIds { ids, limit }).await?;
/// // -> Vec<PersonEntity>
/// ```
#[proc_macro_attribute]
pub fn select_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::select::expand(args.into(), input.into()).into()
}

/// Apply this attribute to a struct to generate a delete query.
/// ```rust,ignore
/// #[delete_query(
///    query = "delete from person where id = ?",
/// )]
/// pub struct DeletePersonById {
///    pub id: Uuid,
/// }
/// ```
#[proc_macro_attribute]
pub fn delete_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::delete::expand(args.into(), input.into()).into()
}

/// Apply this attribute to a entity struct to generate an upsert query.
/// ```rust,ignore
/// #[upsert_query(table = "person", name = UpsertPerson)]
/// #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
/// pub struct PersonEntity {
///     #[pk]
///     pub id: uuid::Uuid,
///     pub email: String,
///     pub created_at: i64,
/// }
/// ```
#[proc_macro_attribute]
pub fn upsert_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::upsert::expand(args.into(), input.into()).into()
}

/// Implements [`scyllax::EntityExt`](scyllax::EntityExt) for the struct.
#[proc_macro_derive(Entity, attributes(rename, pk))]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    entity::expand(input.into()).into()
}

/// Shorthand for applying derive macros on an entity. Essentially:
/// ```rust,ignore
/// #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
/// #input
/// ```
#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    entity::expand_attr(args.into(), input.into()).into()
}

/// Implements
/// * [`scylla::frame::value::Value`] and
/// * [`scylla::cql_to_rust::FromCqlVal`]
///
/// for a struct.
#[proc_macro_derive(JsonData, attributes(rename))]
pub fn json_derive(input: TokenStream) -> TokenStream {
    json::expand(input.into()).into()
}

/// Shorthand for applying derive macros on a JSON body. Essentially:
/// ```rust,ignore
/// #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, scyllax::prelude::Json)]
/// #input
/// ```
#[proc_macro_attribute]
pub fn json_data(args: TokenStream, input: TokenStream) -> TokenStream {
    json::expand_attr(args.into(), input.into()).into()
}

/// Implements
/// * [`scylla::frame::value::Value`] and
/// * [`scylla::cql_to_rust::FromCqlVal`]
/// * [`TryFrom<i32>`]
///
/// for an enum. See [`int_enum`] for more.
#[proc_macro_derive(IntEnum, attributes(rename))]
pub fn int_enum_derive(input: TokenStream) -> TokenStream {
    r#enum::expand(input.into()).into()
}

/// Sets up a scylla-and-protobuf compatible enum.
#[proc_macro_attribute]
pub fn int_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    r#enum::expand_attr(args.into(), input.into()).into()
}
