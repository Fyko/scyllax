use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use scyllax_parser::{select::parse_select, SelectQuery, Value, Variable};
use syn::{DeriveInput, Ident, ItemStruct, Type};

use crate::queries::impl_generic_query;

#[derive(Debug, PartialEq, FromField)]
#[darling(attributes(read_query))]
pub struct ReadQueryDeriveVariable {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(default)]
    pub coalesce_shard_key: bool,
    pub hash_fn: Option<syn::Path>,
}

#[derive(Debug, PartialEq, FromDeriveInput)]
#[darling(attributes(read_query), supports(struct_named))]
pub struct ReadQueryDerive {
    pub ident: syn::Ident,
    pub data: ast::Data<(), ReadQueryDeriveVariable>,

    #[darling(default)]
    pub query: Option<String>,
    #[darling(default)]
    pub query_nocheck: Option<String>,
    pub return_type: syn::Type,
    #[darling(default)]
    pub disable_coalescing: bool,
}

pub fn expand(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };

    let args = match ReadQueryDerive::from_derive_input(&parsed_input) {
        Ok(i) => i,
        Err(e) => return e.write_errors(),
    };
    let fields = args
        .data
        .take_struct()
        .expect("Should never be enum")
        .fields;

    if args.query.is_none() && args.query_nocheck.is_none() {
        return syn::Error::new_spanned(input, "Either query or query_nocheck must be specified")
            .to_compile_error();
    }
    let return_type = args.return_type;
    let struct_ident = args.ident;
    let r#struct = syn::parse2::<ItemStruct>(input.clone()).unwrap();

    // trimmed return_type
    // eg: Vec<OrgEntity> -> OrgEntity
    // eg: OrgEntity -> OrgEntity
    let inner_entity_type = if let syn::Type::Path(path) = return_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            let args = &last_segment.arguments;
            if let syn::PathArguments::AngleBracketed(args) = args {
                let args = &args.args;
                if args.len() != 1 {
                    return syn::Error::new_spanned(
                        return_type,
                        "return_type must be a path with one generic argument",
                    )
                    .to_compile_error();
                }

                if let syn::GenericArgument::Type(ty) = args.first().unwrap() {
                    ty.clone()
                } else {
                    return syn::Error::new_spanned(
                        return_type,
                        "return_type must be a path with one generic argument",
                    )
                    .to_compile_error();
                }
            } else {
                return syn::Error::new_spanned(
                    return_type,
                    "return_type must be a path with one generic argument",
                )
                .to_compile_error();
            }
        } else {
            return_type.clone()
        }
    } else {
        return syn::Error::new_spanned(return_type, "return_type must be a path")
            .to_compile_error();
    };

    // if return_type is a Vec, return type is Vec<return_type>
    // if return_type is not a Vec, return type is Option<return_type>
    let impl_return_type = if let syn::Type::Path(path) = return_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            quote! {
                #return_type
            }
        } else {
            quote! {
                Option<#return_type>
            }
        }
    } else {
        return syn::Error::new_spanned(return_type, "return_type must be a path")
            .to_compile_error();
    };

    // if return_type is a Vec, we need to use the macro scyllax:match_rows!(res, return_type)
    // if return_type is not a Vec, we need to use the macro scyllax:match_row!(res, return_type)
    // eg: Vec<OrgEntity> -> scyllax:match_rows!(res, OrgEntity)
    // eg: OrgEntity -> scyllax:match_row!(res, OrgEntity)
    let (vec_response, parser) = if let syn::Type::Path(path) = return_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            (
                true,
                quote! {
                    scyllax::match_rows!(res, #inner_entity_type)
                },
            )
        } else {
            (
                false,
                quote! {
                    scyllax::match_row!(res, #inner_entity_type)
                },
            )
        }
    } else {
        return syn::Error::new_spanned(return_type, "return_type must be a path")
            .to_compile_error();
    };

    // query parsing
    let query = if let Some(query) = args.query {
        match parse_query(&r#struct, &query, vec_response) {
            Ok(_) => (),
            Err(e) => return e.to_compile_error(),
        };

        query
    } else if let Some(query) = args.query_nocheck {
        query
    } else {
        unreachable!()
    };

    let impl_query = impl_generic_query(&r#struct, query, Some(&inner_entity_type));

    let shard_keys = fields
        .iter()
        .filter(|v| v.coalesce_shard_key)
        .collect::<Vec<&ReadQueryDeriveVariable>>();
    let shard_key: Vec<TokenStream> = if !shard_keys.is_empty() {
        shard_keys
            .iter()
            .map(|sk| {
                if let Some(hash_fn) = &sk.hash_fn {
                    let ident = sk.ident.as_ref().unwrap();
                    quote! {
                        #hash_fn(&self.#ident).hash(state);
                    }
                } else {
                    let ident = sk.ident.as_ref().unwrap();
                    quote! {
                        self.#ident.hash(state);
                    }
                }
            })
            .collect::<Vec<TokenStream>>()
    } else {
        // by default, if there are no shard keys specified, do every field
        fields
            .iter()
            .map(|v| v.ident.as_ref().unwrap())
            .map(|sk| quote! { self.#sk.hash(state); })
            .collect::<Vec<TokenStream>>()
    };

    let should_coalesce = if args.disable_coalescing {
        quote! {
            fn coalesce() -> bool {
                false
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #impl_query

        impl std::hash::Hash for #struct_ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #(#shard_key)*
            }
        }

        #[scyllax::prelude::async_trait]
        impl scyllax::prelude::ReadQuery for #struct_ident {
            type Output = #impl_return_type;

            async fn parse_response(res: scylla::QueryResult) ->
                Result<Self::Output, scyllax::prelude::ScyllaxError>
            {
                #parser
            }

            #should_coalesce
        }
    }
}

fn parse_query(
    input: &ItemStruct,
    query: &String,
    vec_response: bool,
) -> Result<SelectQuery, syn::Error> {
    let (rest, parsed) = match parse_select(query) {
        Ok(parsed) => parsed,
        Err(e) => {
            return Err(syn::Error::new_spanned(
                query.into_token_stream(),
                format!("Failed to parse query: {:#?}", e),
            ))
        }
    };

    if !rest.is_empty() {
        return Err(syn::Error::new_spanned(
            query.into_token_stream(),
            format!("Failed to parse query, stopped at: {:#?}.\nThe parser's still in development... If you're positive it's valid, rename `query` to `query_nockeck`.", rest),
        ));
    }

    // only allow named variables in parsed.conditions. no placeholders.
    if parsed
        .condition
        .iter()
        .any(|condition| matches!(condition.value, Value::Variable(Variable::Placeholder)))
    {
        return Err(syn::Error::new_spanned(
            query.into_token_stream(),
            "Cannot use placeholder variables in query",
        ));
    }

    // check that all variables in parsed.conditions match a field in the struct
    let misses = parsed
        .condition
        .iter()
        .filter_map(|condition| match condition.value {
            Value::Variable(Variable::NamedVariable(ref name)) => {
                if !input
                    .fields
                    .iter()
                    .any(|f| f.ident.as_ref().unwrap() == name)
                {
                    Some(name.clone())
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    if !misses.is_empty() {
        return Err(syn::Error::new_spanned(
            query.into_token_stream(),
            format!(
                "Query contains variables that do not match any fields in the struct: {}",
                misses.join(", ")
            ),
        ));
    }

    if let Some(limit) = parsed.limit.as_ref() {
        if vec_response {
            // if the limit is hardcoded, throw a warning.
            if let Value::Number(value) = limit {
                return Err(syn::Error::new_spanned(
                    query.into_token_stream(),
                    format!(
                        "Query contains a hard-coded `limit` variable ({}) but the return type is a Vec. Consider using a named variable: `pub row_limit: i32`",
                        value
                    ),
                ));
            }
        }

        if !vec_response {
            // if the limit is set to a variable, throw a warning.
            if let Value::Variable(variable) = limit {
                let variable_type = match variable {
                    Variable::NamedVariable(_) => "named",
                    Variable::Placeholder => "placeholder",
                };

                return Err(syn::Error::new_spanned(
                    query.into_token_stream(),
                    format!(
                        "Query contains a {variable_type} `limit` variable ({variable}) but the return type is not a Vec. Consider using a hardcoded value: `limit 1`.",
                    ),
                ));
            }
        }
    }

    Ok(parsed)
}
