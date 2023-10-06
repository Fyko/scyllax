use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::ItemEnum;

pub fn expand_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    quote! {
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, scyllax::prelude::IntEnum)]
        #input
    }
}

/// Enum for source type
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum EmoteProvider {
//     Custom = 0,
//     BTTV = 1,
//     SevenTV = 2,
// }

// impl TryFrom<i32> for EmoteProvider {
//     type Error = Box<dyn std::error::Error>;

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(EmoteProvider::Custom),
//             1 => Ok(EmoteProvider::BTTV),
//             2 => Ok(EmoteProvider::SevenTV),
//             _ => Err(anyhow::anyhow!("Invalid EmoteProvider").into()),
//         }
//     }
// }

// impl EmoteProvider {
//     fn to_int(&self) -> i32 {
//         match self {
//             Self::Custom => 0,
//             Self::BTTV => 1,
//             Self::SevenTV => 2,
//         }
//     }
// }

// impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for EmoteProvider {
//     fn from_cql(
//         cql_val: scylla::frame::response::result::CqlValue,
//     ) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
//         let data = <i32 as scylla::cql_to_rust::FromCqlVal<
//             scylla::frame::response::result::CqlValue,
//         >>::from_cql(cql_val)?;

//         EmoteProvider::try_from(data).map_err(|_| scylla::cql_to_rust::FromCqlValError::BadVal)
//     }
// }

// impl scylla::frame::value::Value for EmoteProvider {
//     fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), scylla::frame::value::ValueTooBig> {
//         <i32 as scylla::frame::value::Value>::serialize(&self.to_int(), buf)
//     }
// }

pub fn expand(input: TokenStream) -> TokenStream {
    let input: ItemEnum = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };
    let ident = &input.ident;

    for variant in input.variants.iter() {
        match &variant.discriminant {
            Some(_) => (),
            None => {
                return syn::Error::new_spanned(
                    variant.into_token_stream(),
                    "Enum variants must have an explicit discriminant, for example `Custom = 0`",
                )
                .into_compile_error();
            }
        }
    }

    let try_from_fields = input.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_value = match &variant.discriminant {
            Some((_, expr)) => expr,
            None => unreachable!(),
        };
        quote! {
            #variant_value => Ok(#ident::#variant_ident),
        }
    });

    let try_from = quote! {
        impl std::convert::TryFrom<i32> for #ident {
            type Error = Box<dyn std::error::Error>;

            #[doc = "Converts integer values to enum values"]
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    #(#try_from_fields)*
                    _ => Err(anyhow::anyhow!("Invalid #ident").into()),
                }
            }
        }
    };

    let to_int_fields = input.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_value = match &variant.discriminant {
            Some((_, expr)) => expr,
            None => unreachable!(),
        };
        quote! {
            Self::#variant_ident => #variant_value,
        }
    });

    let to_int = quote! {
        impl #ident {
            #[doc = "Converts enum values to integer value"]
            fn to_int(&self) -> i32 {
                match self {
                    #(#to_int_fields)*
                }
            }
        }
    };

    let scylla = quote! {
        impl scylla::cql_to_rust::FromCqlVal<scylla::frame::response::result::CqlValue> for #ident {
            fn from_cql(
                cql_val: scylla::frame::response::result::CqlValue,
            ) -> Result<Self, scylla::cql_to_rust::FromCqlValError> {
                let data = <i32 as scylla::cql_to_rust::FromCqlVal<
                    scylla::frame::response::result::CqlValue,
                >>::from_cql(cql_val)?;

                #ident::try_from(data).map_err(|_| scylla::cql_to_rust::FromCqlValError::BadVal)
            }
        }

        impl scylla::frame::value::Value for #ident {
            fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), scylla::frame::value::ValueTooBig> {
                <i32 as scylla::frame::value::Value>::serialize(&self.to_int(), buf)
            }
        }
    };

    quote! {
        #try_from

        #to_int

        #scylla
    }
}
