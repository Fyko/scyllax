//! Scyllax macros. See the scyllax for more information.
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod entity;
mod queries;

pub(crate) fn token_stream_with_error(mut tokens: TokenStream2, error: syn::Error) -> TokenStream2 {
    tokens.extend(error.into_compile_error());
    tokens
}

/// Apply this attribute to a struct to generate a select query.
/// ```rust,ignore
/// #[select_query(
///     query = "select * from person where id = ? limit 1",
///     entity_type = "PersonEntity"
/// )]
/// pub struct GetPersonById {
///     pub id: Uuid,
/// }
/// ```
#[proc_macro_attribute]
pub fn select_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::select::expand(args.into(), input.into()).into()
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
