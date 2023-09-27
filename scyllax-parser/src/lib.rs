//! A parser for CQL queries
//! See the source code and tests for examples of usage (for now).
pub mod common;
pub mod delete;
pub mod select;
pub mod r#where;

pub use common::{Column, Value, Variable};

use delete::DeleteQuery;
use nom::{error::Error, Err, IResult};
use select::SelectQuery;

/// Represents a query
#[derive(Debug, PartialEq)]
pub enum Query {
    /// A select query
    Select(SelectQuery),
    /// A delete query
    Delete(DeleteQuery),
}

fn parse_query(input: &str) -> IResult<&str, Query> {
    nom::branch::alt((
        nom::combinator::map(select::parse_select, Query::Select),
        nom::combinator::map(delete::parse_delete, Query::Delete),
    ))(input)
}

impl<'a> TryFrom<&'a str> for Query {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse_query(value)?.1)
    }
}
