//! This module contains the `prepare_queries!` macro.
use proc_macro2::TokenStream;
use quote::quote;
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
    pub queries: Vec<syn::Ident>,
}

impl Parse for PrepareQueriesInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let queries = input
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

        Ok(Self { name, queries })
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
    let queries = args.queries;
    let name = args.name;

    let stmts = queries.iter().map(|field| {
        let doc = format!("The prepared statement for `{}`.", field);
        quote! {
            #[allow(non_snake_case)]
            #[doc = #doc]
            pub #field: scylla::statement::prepared_statement::PreparedStatement,
        }
    });

    let gets = queries.iter().map(|field| {
		quote! {
			impl scyllax::GetPreparedStatement<#field> for #name {
				#[doc = "Get a prepared statement."]
				fn get_prepared_statement(&self) -> &scylla::statement::prepared_statement::PreparedStatement {
					&self.#field
				}
			}
		}
	});

    let prepares = queries.iter().map(|field| {
        quote! {
            #field: session.prepare(#field::query()).await?,
        }
    });

    quote! {
        #[doc = "A collection of prepared statements."]
        #[allow(non_snake_case)]
        pub struct #name {
            #(#stmts)*
        }

        #[scyllax::async_trait]
        #[doc = "A collection of prepared statements."]
        impl scyllax::Queries for #name {
            async fn new(session: &scylla::Session) -> Result<Self, scyllax::ScyllaxError> {
                Ok(Self {
                    #(#prepares)*
                })
            }

            #[doc = "Get a prepared statement."]
            fn get<T>(&self) -> &scylla::statement::prepared_statement::PreparedStatement
            where
                Self: scyllax::GetPreparedStatement<T>,
            {
                <Self as scyllax::GetPreparedStatement<T>>::get_prepared_statement(self)
            }
        }

        #(#gets)*
    }
}
