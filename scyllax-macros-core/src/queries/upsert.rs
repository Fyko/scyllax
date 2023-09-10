//! Upsert query macro
use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, ItemStruct};

use crate::{entity::get_field_name, token_stream_with_error};

/// Options for the `#[upsert_query]` attribute macro.
#[derive(FromMeta)]
pub struct UpsertQueryOptions {
    /// The name of the upsert struct
    pub name: syn::Ident,
    /// The name of the table to upsert into
    pub table: String,
    /// The struct that holds the query cache.
    pub query_cache: syn::Ident,
}

/// Attribute expand
/// Just adds the dervie macro to the struct.
pub fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.clone()) {
        Ok(args) => args,
        Err(e) => return darling::Error::from(e).write_errors(),
    };

    let args = match UpsertQueryOptions::from_list(&attr_args) {
        Ok(o) => o,
        Err(e) => return e.write_errors(),
    };

    let input: ItemStruct = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return token_stream_with_error(input, e),
    };

    let input_clone = input.clone();
    let pks = input_clone
        .fields
        .iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("pk")))
        .collect::<Vec<_>>();

    if pks.is_empty() {
        return token_stream_with_error(
            input.clone().into_token_stream(),
            syn::Error::new_spanned(
                input.clone().into_token_stream(),
                "Entity can only be derived for structs with at least one #[pk] field.",
            ),
        );
    }

    upsert_impl(&input, &args, &pks)
}

// TODO: handle when all keys are pks and we need to use `insert` instead of `update`
/// Create the implementation for the upsert query
fn upsert_impl(input: &ItemStruct, opt: &UpsertQueryOptions, pks: &[&Field]) -> TokenStream {
    let upsert_struct = &opt.name;
    let upsert_table = &opt.table;
    let struct_ident = &input.ident;

    let fields = match &input.fields {
        syn::Fields::Named(fields) => fields.named.iter().collect::<Vec<_>>(),
        _ => panic!("Entity can only be derived for structs."),
    };

    let expanded_pks = pks
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            let comment = format!("The {} of the {}", ident.as_ref().unwrap(), struct_ident);

            quote! {
                #[doc = #comment]
                pub #ident: #ty
            }
        })
        .collect::<Vec<_>>();

    let maybe_unset_fields = fields
        .iter()
        .filter(|f| !pks.contains(f))
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            let comment = format!("The {} of the {}", ident.as_ref().unwrap(), struct_ident);

            quote! {
                #[doc = #comment]
                pub #ident: scyllax::prelude::MaybeUnset<#ty>
            }
        })
        .collect::<Vec<_>>();

    let docs = format!(
        "Upserts a {} into the `{}` table",
        struct_ident, upsert_table
    );
    let expanded_upsert_struct = quote! {
        #[doc = #docs]
        #[derive(Debug, Clone)]
        pub struct #upsert_struct {
            #(#expanded_pks,)*
            #(#maybe_unset_fields,)*
        }
    };

    // SET clauses
    // expanded variables will loop over every field that isn't Pk
    let (set_columns, set_sv_push) = fields
        .iter()
        // filter out pks
        .filter(|f| !pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(f);
            let errors = error_switchback(f);
            let ident_string = ident.to_string();

            (
                (col.clone(), ident_string.clone()),
                quote! {
                    match variables.add_named_value(#ident_string, &self.#ident) {
                        Ok(_) => (),
                        #errors
                    };
                },
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    // WHERE clauses
    let (where_columns, where_sv_push) = fields
        .iter()
        // filter out pks
        .filter(|f| pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(f);
            let errors = error_switchback(f);
            let ident_string = ident.to_string();

            (
                (col.clone(), ident_string.clone()),
                quote! {
                    match variables.add_named_value(#ident_string, &self.#ident) {
                        Ok(_) => (),
                        #errors
                    };
                },
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let mut query = format!("update {upsert_table} set");
    for (col, named_value) in set_columns {
        query.push_str(&format!(" {col} = :{named_value},"));
    }
    query.pop();
    query.push_str(" where");
    for (col, named_value) in where_columns {
        query.push_str(&format!(" {col} = :{named_value},"));
    }
    query.pop();
    query.push(';');

    let query_cache = &opt.query_cache;
    quote! {
        #input

        #expanded_upsert_struct

        impl scyllax::GenericQuery<#struct_ident> for #upsert_struct {
            fn query() -> String {
                #query.to_string()
            }
        }

        #[scyllax::async_trait]
        impl scyllax::UpsertQuery<#struct_ident, #query_cache> for #upsert_struct {
            fn create_serialized_values(
                &self,
            ) -> Result<scyllax::prelude::SerializedValues, scyllax::BuildUpsertQueryError> {
                let mut variables = scylla::frame::value::SerializedValues::new();

                #(#set_sv_push)*
                #(#where_sv_push)*

                Ok(variables)
            }


            async fn execute(self, db: &scyllax::Executor<#query_cache>) -> Result<scyllax::QueryResult, scyllax::ScyllaxError> {
                let statement = db.queries.get_statement::<#struct_ident>();
                let values = Self::create_serialized_values(&self)?;

                tracing::debug! {
                    query = ?query,
                    values = values.len(),
                    "executing upsert"
                };
                db.session.execute(statement, values).await.map_err(|e| e.into())
            }
        }
    }
}

fn error_switchback(f: &&syn::Field) -> TokenStream {
    let ident = &f.ident;

    quote! {
        Err(scylla::frame::value::SerializeValuesError::TooManyValues) => {
            return Err(scyllax::BuildUpsertQueryError::TooManyValues {
                field: stringify!(#ident).to_string(),
            })
        }
        Err(scylla::frame::value::SerializeValuesError::MixingNamedAndNotNamedValues) => {
            return Err(scyllax::BuildUpsertQueryError::MixingNamedAndNotNamedValues)
        }
        Err(scylla::frame::value::SerializeValuesError::ValueTooBig(_)) => {
            return Err(scyllax::BuildUpsertQueryError::ValueTooBig {
                field: stringify!(#ident).to_string(),
            })
        }
        Err(scylla::frame::value::SerializeValuesError::ParseError) => {
            return Err(scyllax::BuildUpsertQueryError::ParseError {
                field: stringify!(#ident).to_string(),
            })
        }
    }
}
