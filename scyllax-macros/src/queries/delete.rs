use darling::{export::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::token_stream_with_error;

#[derive(FromMeta)]
pub(crate) struct SelectQueryOptions {
    query: String,
    entity_type: syn::Type,
}

pub(crate) fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.clone()) {
        Ok(args) => args,
        Err(e) => return darling::Error::from(e).write_errors(),
    };

    let args = match SelectQueryOptions::from_list(&attr_args) {
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

    quote! {
        #[derive(scylla::ValueList, std::fmt::Debug, std::clone::Clone, PartialEq, Hash)]
        #input

        #[scyllax::async_trait]
        impl scyllax::DeleteQuery<#entity_type> for #struct_ident {
            fn query() -> String {
                #query.to_string()
            }

            async fn prepare(db: &Executor) -> Result<scylla::prepared_statement::PreparedStatement, scylla::transport::errors::QueryError> {
                let query = Self::query();
                tracing::debug!{
                    target = stringify!(#struct_ident),
                    query,
                    "preparing query"
                };
                db.session.add_prepared_statement(&scylla::query::Query::new(query)).await
            }

            async fn execute(self, db: &scyllax::Executor) -> anyhow::Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
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
