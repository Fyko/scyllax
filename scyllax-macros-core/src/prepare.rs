//! This module contains the `prepare_queries!` macro.
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, format_ident};
use syn::{
    parse::{Parse, ParseStream},
    ExprArray,
};

// // prepare_queries!(PersonQueries, [GetPersonById, GetPeopleByIds, DeletePersonById, ...]);
/// Options for the `prepare_queries!` macro.
pub struct PrepareQueriesInput {
    /// The name of the struct to generate.
    pub name: syn::Ident,
    /// The queries to attach to the struct.
    pub read_queries: Vec<syn::Ident>,
    /// Write queries to attach to the struct.
    pub write_queries: Vec<syn::Ident>,
}

impl Parse for PrepareQueriesInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let read_queries = input
            .parse::<ExprArray>()?
            .elems
            .iter()
            .map(|expr| {
                if let syn::Expr::Path(path) = expr {
                    Ok(path.path.get_ident().unwrap().clone())
                } else {
                    Err(syn::Error::new_spanned(expr, "expected an identifier"))
                }
            })
            .collect::<syn::Result<Vec<_>>>()?;

        input.parse::<syn::Token![,]>()?;
        let write_queries = input
            .parse::<ExprArray>()?
            .elems
            .iter()
            .map(|expr| {
                if let syn::Expr::Path(path) = expr {
                    Ok(path.path.get_ident().unwrap().clone())
                } else {
                    Err(syn::Error::new_spanned(expr, "expected an identifier"))
                }
            })
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self { name, read_queries, write_queries })
    }
}

// prepare_queries!(PersonQueries, [GetPersonById, GetPeopleByIds, DeletePersonById, ...]);
// creates a struct like this:
// pub struct PersonQueries {
//   GetPersonById: scylla::statement::prepared_statement::PreparedStatement,
//   GetPeopleByIds: scylla::statement::prepared_statement::PreparedStatement,
//   DeletePersonById: scylla::statement::prepared_statement::PreparedStatement,
//   ...
// }
/// Expands the `prepare_queries!` macro.
pub fn expand(input: TokenStream) -> TokenStream {
    let args: PrepareQueriesInput = match syn::parse2(input) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };
    let read_queries = args.read_queries;
    let write_queries = args.write_queries;
    let queries: Vec<&proc_macro2::Ident> = read_queries.iter().chain(write_queries.iter()).collect();
    let name = args.name;

    let prepared_statements = queries.iter().map(|field| {
        let doc = format!("The prepared statement for `{}`.", field);
        quote! {
            #[allow(non_snake_case)]
            #[doc = #doc]
            pub #field: scylla_reexports::PreparedStatement,
        }
    });

    let get_prepared_statements = queries.iter().map(|field| {
        quote! {
            impl scyllax::prelude::GetPreparedStatement<#field> for #name {
                #[doc = "Get a prepared statement."]
                fn get(&self) -> &scyllax::prelude::scylla_reexports::PreparedStatement {
                    &self.#field
                }
            }
        }
    });

    let prepares = queries.iter().map(|field| {
        quote! {
            #field: scyllax::prelude::prepare_query(&session, #field::query()).await?,
        }
    });

    let coalescing_senders = read_queries.iter().map(|field| {
        let doc = format!("The task for `{}`.", field);
        let field = format_ident!("{}_task", field).to_token_stream();
        quote! {
            #[allow(non_snake_case)]
            #[doc = #doc]
            pub #field: tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<'_, #field>>,
        }
    });

    let get_coalescing_senders = read_queries.iter().map(|field| {
        quote! {
            impl scyllax::prelude::GetCoalescingSender<#field> for #name {
                #[doc = "Get a task."]
                fn get(&self) -> &tokio::sync::mpsc::Sender<scyllax::executor::ShardMessage<'_, #field>> {
                    &self.#field
                }
            }
        }
    });

    let create_senders = read_queries.iter().map(|field| {
        let field = format_ident!("{}_task", field);
        quote! {
            #field: {
                let (tx, rx) = tokio::sync::mpsc::channel(100);
                let queries = self.clone();
                tokio::spawn(self.read_task(rx));
                tx
            },
        }
    });

    quote! {
        #[doc = "A collection of prepared statements."]
        #[allow(non_snake_case)]
        pub struct #name {
            #(#prepared_statements)*
            #(#coalescing_senders)*
        }

        #[scyllax::prelude::async_trait]
        #[doc = "A collection of prepared statements."]
        impl scyllax::prelude::QueryCollection for #name {
            async fn new(session: &scylla::Session) -> Result<Self, scyllax::prelude::ScyllaxError> {
                Ok(Self {
                    #(#prepares)*,
                    #(#create_senders)*
                })
            }
        }

        #(#get_prepared_statements)*
        #(#get_coalescing_senders)*
    }
}
