use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub mod read;
pub mod upsert;
pub mod write;

/// Implements the [`Query`] trait for a struct.
pub fn impl_generic_query(
    input: &ItemStruct,
    query: String,
    inner_entity_type: Option<&syn::Type>,
) -> TokenStream {
    let struct_ident = &input.ident;

    let query = if let Some(inner_entity_type) = inner_entity_type {
        quote!(#query.replace("*", &#inner_entity_type::keys().join(", ")))
    } else {
        quote!(#query.to_string())
    };

    quote! {
        #[scyllax::prelude::async_trait]
        impl scyllax::prelude::Query for #struct_ident {
            fn query() -> String {
                #query
            }
        }
    }
}
