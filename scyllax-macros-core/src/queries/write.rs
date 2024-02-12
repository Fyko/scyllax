use crate::queries::impl_generic_query;
use darling::{export::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

#[derive(FromMeta)]
pub(crate) struct WriteQueryOptions {
    query: Option<String>,
    query_nocheck: Option<String>,
}

pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.clone()) {
        Ok(args) => args,
        Err(e) => return darling::Error::from(e).write_errors(),
    };

    let args = match WriteQueryOptions::from_list(&attr_args) {
        Ok(o) => o,
        Err(e) => return e.write_errors(),
    };

    let input: ItemStruct = match syn::parse2(item.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };

    let query = if let Some(query) = args.query {
        // match parse_query(&input, &query) {
        //     Ok(_) => (),
        //     Err(e) => return e.to_compile_error(),
        // };

        query
    } else if let Some(query) = args.query_nocheck {
        query
    } else {
        unreachable!()
    };

    let impl_query = impl_generic_query(&input, query, None);
    let struct_ident = &input.ident;

    quote! {
        #[derive(std::fmt::Debug, std::clone::Clone, PartialEq, Hash, scylla::SerializeRow)]
        #input

        #impl_query

        impl scyllax::prelude::WriteQuery for #struct_ident {}
    }
}
