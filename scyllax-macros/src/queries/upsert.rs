use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, ItemStruct};

use crate::{entity::get_field_name, token_stream_with_error};

#[derive(FromMeta)]
pub(crate) struct UpsertQueryOptions {
    pub name: syn::Ident,
    pub table: String,
}

/// Attribute expand
/// Just adds the dervie macro to the struct.
pub(crate) fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
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

    let counters = input_clone
        .fields
        .iter()
        .filter(|f| f.attrs.iter().any(|a| a.path().is_ident("counter")))
        .collect::<Vec<_>>();
    // require the type of every counter be i64
    for counter in &counters {
        if let syn::Type::Path(path) = &counter.ty {
            if let Some(ident) = path.path.get_ident() {
                if ident != "scylla::frame::value::Counter" {
                    return token_stream_with_error(
                        input.into_token_stream(),
                        syn::Error::new_spanned(
                            counter.into_token_stream(),
                            "Counter fields must be of type `scylla::frame::value::Counter`",
                        ),
                    );
                }
            }
        } else {
            return token_stream_with_error(
                input.into_token_stream(),
                syn::Error::new_spanned(
                    counter.into_token_stream(),
                    "Counter fields must be of type `scylla::frame::value::Counter",
                ),
            );
        }
    }

    if pks.is_empty() {
        return token_stream_with_error(
            input.clone().into_token_stream(),
            syn::Error::new_spanned(
                input.clone().into_token_stream(),
                "Entity can only be derived for structs with at least one #[pk] field.",
            ),
        );
    }

    upsert_impl(&input, &args, &pks, &counters)
}

// TODO: handle when all keys are pks and we need to use `insert` instead of `update`
/// Create the implementation for the upsert query
pub(crate) fn upsert_impl(
    input: &ItemStruct,
    opt: &UpsertQueryOptions,
    pks: &[&Field],
    counters: &[&Field],
) -> TokenStream {
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
    let (set_clauses, set_sv_push) = fields
        .iter()
        // filter out pks
        .filter(|f| !pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(f);
            let errors = error_switchback(f);
            let ident_string = ident.to_string();

            let query = if counters.contains(f) {
                format!("{col} = {col} + :{ident_string}")
            } else {
                format!("{col} = :{ident_string}")
            };

            (
                query,
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
    let (where_clauses, where_sv_push) = fields
        .iter()
        // filter out pks
        .filter(|f| pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(f);
            let errors = error_switchback(f);
            let named_var = ident.to_string();

            (
                (col.clone(), named_var.clone()),
                quote! {
                    match variables.add_named_value(#named_var, &self.#ident) {
                        Ok(_) => (),
                        #errors
                    };
                },
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    // if there are no set clauses, then we need to do an insert
    // because we can't do an update with no set clauses
    let query = build_query(upsert_table, set_clauses, where_clauses);

    quote! {
        #input

        #expanded_upsert_struct

        #[scyllax::async_trait]
        impl scyllax::UpsertQuery<#struct_ident> for #upsert_struct {
            fn query(
                &self,
            ) -> Result<(String, scyllax::prelude::SerializedValues), scyllax::BuildUpsertQueryError> {
                let query = #query.to_string();
                let mut variables = scylla::frame::value::SerializedValues::new();

                #(#set_sv_push)*
                #(#where_sv_push)*

                Ok((query, variables))
            }


            async fn execute(self, db: &scyllax::Executor) -> Result<scyllax::QueryResult, scyllax::ScyllaxError> {
                let (query, values) = Self::query(&self)?;

                tracing::debug! {
                    query = ?query,
                    values = values.len(),
                    "executing upsert"
                };
                db.session.execute(query, values).await.map_err(|e| e.into())
            }
        }
    }
}

fn build_query(
    table: &String,
    set_clauses: Vec<String>,
    where_clauses: Vec<(String, String)>,
) -> String {
    if set_clauses.is_empty() {
        let mut query = format!("insert into {table}");
        let (cols, named_var) = where_clauses.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
        let cols = cols.join(", ");
        let named_var = named_var
            .into_iter()
            .map(|var| format!(":{var}"))
            .collect::<Vec<_>>()
            .join(", ");

        query.push_str(&format!(" ({cols}) values ({named_var});"));

        query
    } else {
        let mut query = format!("update {table} set ");
        let query_set = set_clauses.join(", ");
        query.push_str(&query_set);

        query.push_str(" where ");
        let query_where = where_clauses
            .into_iter()
            .map(|(col, ident_string)| format!("{col} = :{ident_string}"))
            .collect::<Vec<_>>()
            .join(" and ");
        query.push_str(&query_where);

        query.push(';');

        query
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

#[cfg(test)]
mod tests {
    use super::build_query;

    fn get_set_clauses() -> Vec<String> {
        vec![
            "name = :name",
            "email = :email",
            "\"createdAt\" = :created_at",
        ]
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
    }

    fn get_where_clauses() -> Vec<(String, String)> {
        vec![("id", "id"), (r#""orgId""#, "org_id")]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_update() {
        let query = build_query(
            &"person".to_string(),
            get_set_clauses(),
            get_where_clauses(),
        );

        assert_eq!(
            query,
            "update person set name = :name, email = :email, \"createdAt\" = :created_at where id = :id and \"orgId\" = :org_id;",
        );
    }

    #[test]
    fn test_insert() {
        let query = build_query(&"person".to_string(), vec![], get_where_clauses());

        assert_eq!(
            query,
            "insert into person (id, \"orgId\") values (:id, :org_id);",
        );
    }
}
