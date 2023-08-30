use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Field, ItemStruct};

use crate::{token_stream_with_error, entity::get_field_name};

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
pub(crate) fn upsert_impl(
    input: &ItemStruct,
    opt: &UpsertQueryOptions,
    pks: &Vec<&Field>,
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

            quote! {
                #ident: #ty
            }
        })
        .collect::<Vec<_>>();

    let maybe_unset_fields = fields
        .iter()
        .filter(|f| !pks.contains(f))
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;

            quote! {
                #ident: scyllax::prelude::MaybeUnset<#ty>
            }
        })
        .collect::<Vec<_>>();

    let expanded_upsert_struct = quote! {
        #[derive(Debug, Clone)]
        struct #upsert_struct {
            #(#expanded_pks,)*
            #(#maybe_unset_fields,)*
        }
    };

    // expanded variables will loop over every field that isn't Pk
    let expanded_variables = fields
        .iter()
        // filter out pks
        .filter(|f| !pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(&f);
            let errors = error_switchback(&f);

            quote! {
                if let scyllax::prelude::MaybeUnset::Set(#ident) = &self.#ident {
                    // FIX: ident
                    fragments.push_str(concat!(stringify!(#col = ?), " "));

                    match variables.add_value(#ident) {
                        Ok(_) => (),
                        #errors
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let expanded_pks = fields
        .iter()
        // filter out pks
        .filter(|f| pks.contains(f))
        .map(|f| {
            let ident = &f.ident.clone().unwrap();
            let col = get_field_name(&f);
            let errors = error_switchback(&f);

            quote! {
                // FIX: ident
                fragments.push_str(concat!(stringify!(where #col = ?, ), " "));
                match variables.add_value(&self.#ident) {
                    Ok(_) => (),
                    #errors
                }
            }
        })
        .collect::<Vec<_>>();

    // return expanded_upsert_struct;

    quote! {
        #expanded_upsert_struct

        #[scyllax::async_trait]
        impl scyllax::UpsertQuery<#struct_ident> for #upsert_struct {
            fn query(
                &self,
            ) -> Result<(String, scyllax::prelude::SerializedValues), scyllax::BuildUpsertQueryError> {
                let mut fragments = String::from(stringify!(update #upsert_table set ));
                let mut variables = scylla::frame::value::SerializedValues::new();

                #(#expanded_variables)*

                fragments.pop();
                fragments.pop();
                #(#expanded_pks)*

                fragments.pop();
                fragments.pop();

                fragments.push_str(";");

                Ok((fragments, variables))
            }


            async fn execute(self, db: &scyllax::Executor) -> anyhow::Result<scyllax::QueryResult, scyllax::QueryError> {
                let (query, values) = Self::query(&self)?;

                db.session.execute(query, values).await.map_err(|e| e.into())
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
