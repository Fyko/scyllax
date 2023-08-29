use proc_macro::TokenStream;

mod entity;
mod queries;

#[proc_macro_attribute]
pub fn select_query(args: TokenStream, input: TokenStream) -> TokenStream {
    queries::select::expand(args.into(), input.into()).into()
}

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
#[proc_macro_derive(Entity, attributes(rename))]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    entity::expand(input)
}
