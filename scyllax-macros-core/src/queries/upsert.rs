use crate::entity::get_field_name;
use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, ItemStruct};

#[derive(FromMeta)]
pub(crate) struct UpsertQueryOptions {
    pub name: syn::Ident,
    pub table: String,
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
        Err(e) => return e.to_compile_error(),
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
                    return syn::Error::new_spanned(
                        counter.into_token_stream(),
                        "Counter fields must be of type `scylla::frame::value::Counter`",
                    )
                    .to_compile_error();
                }
            }
        } else {
            return syn::Error::new_spanned(
                counter.into_token_stream(),
                "Counter fields must be of type `scylla::frame::value::Counter",
            )
            .to_compile_error();
        }
    }

    if pks.is_empty() {
        return syn::Error::new_spanned(
            input.clone().into_token_stream(),
            "Entity can only be derived for structs with at least one #[pk] field.",
        )
        .to_compile_error();
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
            let ident_string = ident.to_string();

            let query = if counters.contains(f) {
                format!("{col} = {col} + :{ident_string}")
            } else {
                format!("{col} = :{ident_string}")
            };

            (
                query,
                quote! {
                    values.add_named_value(#ident_string, &self.#ident)?;
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
            let named_var = ident.to_string();

            (
                (col.clone(), named_var.clone()),
                quote! {
                    values.add_named_value(#named_var, &self.#ident)?;
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

        #[scyllax::prelude::async_trait]
        impl scyllax::prelude::Query for #upsert_struct {
            fn query() -> String {
                #query.to_string()
            }

            fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
                let mut values = scylla_reexports::value::SerializedValues::new();

                #(#set_sv_push)*
                #(#where_sv_push)*

                Ok(values)
            }
        }

        impl scyllax::prelude::WriteQuery for #upsert_struct {}
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
