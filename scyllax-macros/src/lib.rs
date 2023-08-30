use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod entity;
mod queries;

pub(crate) fn token_stream_with_error(mut tokens: TokenStream2, error: syn::Error) -> TokenStream2 {
    tokens.extend(error.into_compile_error());
    tokens
}

#[proc_macro_attribute]
pub fn select_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::select::expand(args.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn upsert_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::upsert::expand(args.into(), input.into()).into()
}

#[proc_macro_derive(Entity, attributes(rename, pk))]
/// Implement functions for the entity.
/// ```rs, ignore
/// #[derive(Debug, Entity)]
/// struct User {
///     id: Uuid,
///     email: String,
///     #[rename("createdAt")]
///     created_at: DateTime<Utc>,
/// }
/// ```
/// `User::keys()` will return `vec!["id", "email", "createdAt"]`
pub fn entity_derive(input: TokenStream) -> TokenStream {
    entity::expand(input.into()).into()
}
