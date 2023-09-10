//! Macros for delete queries.
use darling::{export::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::token_stream_with_error;

/// Options for the `#[delete_query]` attribute macro.
#[derive(FromMeta)]
pub struct DeleteQueryOptions {
    /// The query to execute.
    pub query: String,
    /// The type of the entity to return.
    pub entity_type: syn::Type,
}

/// Expand the `#[delete_query]` attribute macro.
pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.clone()) {
        Ok(args) => args,
        Err(e) => return darling::Error::from(e).write_errors(),
    };

    let args = match DeleteQueryOptions::from_list(&attr_args) {
        Ok(o) => o,
        Err(e) => return e.write_errors(),
    };

    let entity_type = args.entity_type;
    let query = args.query.clone();

    let input: ItemStruct = match syn::parse2(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    let struct_ident = &input.ident;
    let query_cache = concat!(stringify!(#inner_entity_type), "Queries");

    quote! {
        #[derive(scylla::ValueList, std::fmt::Debug, std::clone::Clone, PartialEq, Hash)]
        #input

        impl scyllax::GenericQuery<#entity_type> for #struct_ident {
            fn query() -> String {
                #query.to_string()
            }
        }

        #[scyllax::async_trait]
        impl scyllax::DeleteQuery<#entity_type, #query_cache> for #struct_ident {
            async fn execute(self, db: &scyllax::Executor<#query_cache>) -> anyhow::Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
                let query = Self::query();
                tracing::debug! {
                    query,
                    "executing delete"
                };

                db.session.execute(query, self).await
            }
        }
    }
}
