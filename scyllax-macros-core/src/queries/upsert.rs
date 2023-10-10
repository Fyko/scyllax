use darling::{ast::NestedMeta, FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{DeriveInput, ItemStruct};

use crate::entity::{EntityDerive, EntityDeriveColumn};

#[derive(FromMeta)]
pub(crate) struct UpsertQueryOptions {
    pub name: syn::Ident,
    pub table: String,
    pub ttl: Option<bool>,
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

    let input: DeriveInput = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };

    let entity = match EntityDerive::from_derive_input(&input) {
        Ok(e) => e,
        Err(e) => return e.write_errors(),
    };

    upsert_impl(&input, &args, &entity)
}

/// Create the implementation for the upsert query
pub(crate) fn upsert_impl(
    input: &DeriveInput,
    opt: &UpsertQueryOptions,
    entity: &EntityDerive,
) -> TokenStream {
    let upsert_struct = &opt.name;
    let upsert_table = &opt.table;
    let struct_ident = &input.ident;
    let keys = entity
        .data
        .as_ref()
        .take_struct()
        .expect("Should never be enum")
        .fields;
    let primary_keys: Vec<&&EntityDeriveColumn> = keys.iter().filter(|f| f.primary_key).collect();
    let counters: Vec<&&EntityDeriveColumn> = keys.iter().filter(|f| f.counter).collect();

    let input: ItemStruct = match syn::parse2(input.to_token_stream()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };

    let expanded_pks = primary_keys
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

    let maybe_unset_fields = keys
        .iter()
        .filter(|f| !primary_keys.contains(f))
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


    let ttl = if opt.ttl.unwrap_or(false) {
        quote! {
            #[doc = "The ttl of the row in seconds"]
            pub set_ttl: i32,
        }
    } else {
        quote! {}
    };

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
            #ttl
        }
    };

    // SET clauses
    // expanded variables will loop over every field that isn't Pk
    let (set_clauses, set_sv_push) = keys
        .iter()
        // filter out pks
        .filter(|f| !primary_keys.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = f.name.as_ref().unwrap();
            let ident_string = ident.to_string();

            let query = if counters.contains(&f) {
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
    let (where_clauses, where_sv_push) = keys
        .iter()
        // filter out pks
        .filter(|f| primary_keys.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = f.name.as_ref().unwrap();
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
    let query = build_query(opt, upsert_table, set_clauses, where_clauses);
    let ttl_sv_push = if opt.ttl.unwrap_or(false) {
        quote! {
            values.add_named_value("set_ttl", &self.set_ttl)?;
        }
    } else {
        quote! {}
    };

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
                #ttl_sv_push

                Ok(values)
            }
        }

        impl scyllax::prelude::WriteQuery for #upsert_struct {}
    }
}

fn build_query(
    args: &UpsertQueryOptions,
    table: &String,
    set_clauses: Vec<String>,
    where_clauses: Vec<(String, String)>,
) -> String {
    let ttl = match args.ttl.unwrap_or(false) {
        true => " using ttl :set_ttl",
        _ => "",
    };

    if set_clauses.is_empty() {
        let mut query = format!("insert into {table}{ttl}");
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
        let mut query = format!("update {table}{ttl} set ");
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
    use syn::{parse::Parser, parse_str};

    use super::*;

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
            &UpsertQueryOptions {
                name: syn::parse_str::<syn::Ident>("UpdatePerson").unwrap(),
                table: "person".to_string(),
                ttl: None,
            },
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
    fn test_update_ttl() {
        let query = build_query(
            &UpsertQueryOptions {
                name: syn::parse_str::<syn::Ident>("UpdatePerson").unwrap(),
                table: "person".to_string(),
                ttl: Some(true),
            },
            &"person".to_string(),
            get_set_clauses(),
            get_where_clauses(),
        );

        assert_eq!(
            query,
            "update person using ttl :set_ttl set name = :name, email = :email, \"createdAt\" = :created_at where id = :id and \"orgId\" = :org_id;",
        );
    }

    #[test]
    fn test_insert() {
        let query = build_query(&UpsertQueryOptions {
            name: syn::parse_str::<syn::Ident>("UpdatePerson").unwrap(),
            table: "person".to_string(),
            ttl: Default::default(),
        }, &"person".to_string(), vec![], get_where_clauses());

        assert_eq!(
            query,
            "insert into person (id, \"orgId\") values (:id, :org_id);",
        );
    }

    #[test]
    fn test_insert_ttl() {
        let query = build_query(&UpsertQueryOptions {
            name: syn::parse_str::<syn::Ident>("UpdatePerson").unwrap(),
            table: "person".to_string(),
            ttl: Some(true),
        }, &"person".to_string(), vec![], get_where_clauses());

        assert_eq!(
            query,
            "insert into person using ttl :set_ttl (id, \"orgId\") values (:id, :org_id);",
        );
    }
}
