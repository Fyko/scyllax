use darling::{export::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use scyllax_parser::{select::parse_select, SelectQuery, Value, Variable};
use syn::ItemStruct;

use crate::queries::impl_generic_query;

#[derive(FromMeta)]
pub(crate) struct SelectQueryOptions {
    query: Option<String>,
    query_nocheck: Option<String>,
    return_type: syn::Type,
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

    if args.query.is_none() && args.query_nocheck.is_none() {
        return syn::Error::new_spanned(item, "Either query or query_nocheck must be specified")
            .to_compile_error();
    }

    let return_type = args.return_type;

    let input: ItemStruct = match syn::parse2(item.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };
    let struct_ident = &input.ident;

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

    // query parsing
    let query = if let Some(query) = args.query {
        match parse_query(&input, &query) {
            Ok(_) => (),
            Err(e) => return e.to_compile_error(),
        };

        query
    } else if let Some(query) = args.query_nocheck {
        query
    } else {
        unreachable!()
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
    let parser = if let syn::Type::Path(path) = return_type.clone() {
        let last_segment = path.path.segments.last().unwrap();
        let ident = &last_segment.ident;

        if ident == "Vec" {
            quote! {
                scyllax::match_rows!(res, #inner_entity_type)
            }
        } else {
            quote! {
                scyllax::match_row!(res, #path)
            }
        }
    } else {
        return syn::Error::new_spanned(return_type, "return_type must be a path")
            .to_compile_error();
    };

    let impl_query = impl_generic_query(&input, query, Some(&inner_entity_type));

    quote! {
        #[derive(scylla::ValueList, std::fmt::Debug, std::clone::Clone, PartialEq, Hash)]
        #input

        #impl_query

        #[scyllax::prelude::async_trait]
        impl scyllax::prelude::ReadQuery for #struct_ident {
            type Output = #impl_return_type;

            async fn parse_response(res: scylla::QueryResult) ->
                Result<Self::Output, scyllax::prelude::ScyllaxError>
            {
                #parser
            }
        }
    }
}

fn parse_query(input: &ItemStruct, query: &String) -> Result<SelectQuery, syn::Error> {
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

    // only allow named OR placeholder variables in parsed.conditions, not both.
    // let (has_named, has_placeholder) =
    //     parsed
    //         .condition
    //         .iter()
    //         .fold(
    //             (false, false),
    //             |(named, placeholder), condition| match condition.value {
    //                 Value::Variable(Variable::NamedVariable(_)) => (true, placeholder),
    //                 Value::Variable(Variable::Placeholder) => (named, true),
    //                 _ => (named, placeholder),
    //             },
    //         );

    // if has_named && has_placeholder {
    //     return Err(syn::Error::new_spanned(
    //         query.into_token_stream(),
    //         "Cannot mix named and placeholder variables in query",
    //     ));
    // }

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

    Ok(parsed)
}
