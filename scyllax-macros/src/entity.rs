use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Field};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let keys = match &input.data {
        Data::Struct(s) => s.fields.iter().map(get_field_name).collect::<Vec<_>>(),
        _ => panic!("Entity can only be derived for structs."),
    };

    let expanded = quote! {
        impl scyllax::EntityExt<#name> for #name {
            fn keys() -> Vec<String> {
                vec![#(#keys.to_string()),*]
            }
        }
    };

    TokenStream::from(expanded)
}

/// This is used to get the name of a field, taking into account the `#[rename]` attribute.
fn get_field_name(field: &Field) -> String {
    let rename = field.attrs.iter().find(|a| a.path().is_ident("rename"));
    if let Some(rename) = rename {
        let expr = rename.parse_args::<Expr>().expect("Expected an expression");
        if let Expr::Lit(lit) = expr {
            if let syn::Lit::Str(s) = lit.lit {
                return format!(r##""{}""##, s.value());
            }
        }
    }

    field
        .ident
        .as_ref()
        .expect("Expected field to have a name")
        .to_string()
}
