//! Scyllax macros. See the scyllax for more information.
use proc_macro2::TokenStream;

pub mod entity;
pub mod json;
pub mod queries;

/// Throw an error with the tokens for better error messages.
pub fn token_stream_with_error(mut tokens: TokenStream, error: syn::Error) -> TokenStream {
    tokens.extend(error.into_compile_error());
    tokens
}

/// Expand the `prepare_queries!` macro.
pub fn prepare_queries(input: TokenStream) -> TokenStream {
    queries::prepare::expand(input)
}
