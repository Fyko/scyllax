use darling::{export::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::token_stream_with_error;

#[derive(FromMeta)]
pub struct SelectQueryOptions {
    query: String,
    entity_type: syn::Type,
}

pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.clone()) {
        Ok(args) => args,
        Err(e) => return darling::Error::from(e).write_errors(),
    };

    let args = match SelectQueryOptions::from_list(&attr_args) {
        Ok(o) => o,
        Err(e) => return e.write_errors(),
    };

    let query = args.query.clone();
    let entity_type = args.entity_type;

    let input: ItemStruct = match syn::parse2(item.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(item, e),
    };
    let struct_ident = &input.ident;

    // if entity_type is a Vec, return type is Vec<entity_type>
    // if entity_type is not a Vec, return type is Option<entity_type>
    let return_type = if let syn::Type::Path(path) = entity_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            quote! {
                #entity_type
            }
        } else {
            quote! {
                Option<#entity_type>
            }
        }
    } else {
        return token_stream_with_error(
            item,
            syn::Error::new_spanned(entity_type, "entity_type must be a path"),
        );
    };

    // if entity_type is a Vec, we need to use the macro scyllax:match_rows!(res, entity_type)
    // if entity_type is not a Vec, we need to use the macro scyllax:match_row!(res, entity_type)
    // eg: Vec<OrgEntity> -> scyllax:match_rows!(res, OrgEntity)
    // eg: OrgEntity -> scyllax:match_row!(res, OrgEntity)
    let parser = if let syn::Type::Path(path) = entity_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            quote! {
                scyllax::match_rows!(res, #path)
            }
        } else {
            quote! {
                scyllax::match_row!(res, #path)
            }
        }
    } else {
        return token_stream_with_error(
            item,
            syn::Error::new_spanned(entity_type, "entity_type must be a path"),
        );
    };

    quote! {
        #[derive(scylla::ValueList, std::fmt::Debug, std::clone::Clone, PartialEq, Hash)]
        #input

        #[scyllax::async_trait]
        impl scyllax::SelectQuery<#entity_type, #return_type> for #struct_ident {
            fn query() -> String {
                #query.replace("*", &#entity_type::keys().join(", "))
            }

            async fn prepare(db: &Executor) -> Result<scylla::prepared_statement::PreparedStatement, scylla::transport::errors::QueryError> {
                tracing::debug!("preparing query {}", stringify!(#struct_ident));
                db.session.add_prepared_statement(&scylla::query::Query::new(Self::query())).await
            }

            async fn execute(self, db: &scyllax::Executor) -> anyhow::Result<scylla::QueryResult, scylla::transport::errors::QueryError> {
                let query = Self::query();
                tracing::debug!{
                    query,
                    "executing select"
                };

                db.session.execute(query, self).await
            }

            async fn parse_response(res: scylla::QueryResult) -> Result<#return_type, scyllax::ScyllaxError> {
                #parser
            }
        }
    }
}
