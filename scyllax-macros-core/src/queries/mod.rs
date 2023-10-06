use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub mod read;
pub mod upsert;
pub mod write;

/// Creates the [`Query#bind`] function for any query.
///
/// This will take the query struct create a SerializedValue from each field.
pub fn create_bind_function(item: &ItemStruct) -> TokenStream {
    let sets = item
        .fields
        .iter()
        .map(|field| {
            let field_name = field.ident.clone().unwrap();

            quote! {
                values.add_named_value(stringify!(#field_name), &self.#field_name)?;
            }
        })
        .collect::<Vec<_>>();

    quote! {
        fn bind(&self) -> scyllax::prelude::SerializedValuesResult {
            let mut values = scylla_reexports::value::SerializedValues::new();

            #(#sets)*

            Ok(values)
        }
    }
}

/// Implements the [`Query`] trait for a struct.
pub fn impl_generic_query(
    input: &ItemStruct,
    query: String,
    inner_entity_type: Option<&syn::Type>,
) -> TokenStream {
    let struct_ident = &input.ident;
    let bind_fn = create_bind_function(input);

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

            #bind_fn
        }
    }
}
