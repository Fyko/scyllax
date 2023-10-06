use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn expand_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    quote! {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, scyllax::prelude::JsonData)]
        #input
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input: ItemStruct = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };
    let ident = &input.ident;

    let expanded = quote! {
        impl scylla::frame::value::Value for #ident {
            fn serialize(
                &self,
                buf: &mut Vec<u8>
            ) -> Result<(), scylla::frame::value::ValueTooBig> {
                let data = serde_json::to_vec(self).unwrap();
                <Vec<u8> as scylla::frame::value::Value>::serialize(&data, buf)
            }
        }

        impl scylla::cql_to_rust::FromCqlVal<
            scylla::frame::response::result::CqlValue
        > for #ident {
            fn from_cql(
                cql_val: scylla::frame::response::result::CqlValue
            ) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                let data = <String as scylla::cql_to_rust::FromCqlVal<
                    scylla::frame::response::result::CqlValue>
                >::from_cql(cql_val)?;

                serde_json::from_str(&data)
                    .ok()
                    .ok_or(scylla::cql_to_rust::FromCqlValError::BadCqlType)
            }
        }
    };

    expanded
}
