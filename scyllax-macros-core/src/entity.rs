use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, DeriveInput, Ident, Type};

#[derive(Debug, PartialEq, FromField)]
#[darling(attributes(entity), and_then = EntityDeriveColumn::set_name)]
pub struct EntityDeriveColumn {
    pub ident: Option<Ident>,
    pub ty: Type,
    /// this is required to be set as optional and default
    /// because we set it manually in [`EntityDeriveColumn::set_name`]. It will NEVER be None.
    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub counter: bool,
    #[darling(default)]
    pub primary_key: bool,
    #[darling(default)]
    pub rename: Option<String>,
}

impl EntityDeriveColumn {
    /// Sets the name of the column, considering the rename attribute.
    fn set_name(self) -> darling::Result<Self> {
        let Self {
            ident,
            ty,
            counter,
            primary_key,
            rename,
            ..
        } = self;

        let name = rename
            .clone()
            .or_else(|| ident.as_ref().map(|i| i.to_string()))
            .map(|i| format!(r##""{i}""##));

        Ok(Self {
            ident,
            ty,
            name,

            counter,
            primary_key,
            rename,
        })
    }
}

#[derive(Debug, PartialEq, FromDeriveInput)]
#[darling(attributes(entity), supports(struct_named))]
pub struct EntityDerive {
    pub ident: Ident,
    pub data: ast::Data<util::Ignored, EntityDeriveColumn>,
}

impl ToTokens for EntityDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EntityDerive {
            ref ident,
            ref data,
        } = *self;

        let fields = data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields;

        // validate counters
        for field in fields.iter().filter(|f| f.counter) {
            if let syn::Type::Path(path) = &field.ty {
                if let Some(ident) = path.path.get_ident() {
                    if ident != "scylla::frame::value::Counter" {
                        tokens.extend(
                            syn::Error::new(
                                field.ident.span(),
                                "Counter fields must be of type `scylla::frame::value::Counter`",
                            )
                            .to_compile_error(),
                        );

                        return;
                    }
                }
            } else {
                tokens.extend(
                    syn::Error::new(
                        field.ident.span(),
                        "Counter fields must be of type `scylla::frame::value::Counter`",
                    )
                    .to_compile_error(),
                );

                return;
            }
        }

        let keys: Vec<TokenStream> = fields
            .iter()
            .map(|f| {
                let name = &f.name;
                quote!(#name.to_string())
            })
            .collect();

        let primary_keys: Vec<TokenStream> = fields
            .iter()
            .filter(|f| f.primary_key)
            .map(|f| {
                let name = &f.name;
                quote!(#name.to_string())
            })
            .collect();

        let spat = quote! {
            impl scyllax::prelude::EntityExt<#ident> for #ident {
                fn keys() -> Vec<String> {
                    vec![#(#keys),*]
                }

                fn pks() -> Vec<String> {
                    vec![#(#primary_keys),*]
                }
            }
        };

        tokens.extend(spat);
    }
}

/// Attribute expand
/// Just adds the dervie macro to the struct.
pub fn expand(input: TokenStream) -> TokenStream {
    let input: DeriveInput = match syn::parse2(input.clone()) {
        Ok(it) => it,
        Err(e) => return e.to_compile_error(),
    };

    match EntityDerive::from_derive_input(&input) {
        Ok(e) => e.into_token_stream(),
        Err(e) => e.write_errors(),
    }
}

/// Expands the shorthand attribute
pub fn expand_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    quote! {
        #[derive(
            Clone,
            Debug,
            PartialEq,
            scylla_reexports::FromRow,
            scylla_reexports::ValueList,
            scyllax::prelude::Entity
        )]
        #input
    }
}
