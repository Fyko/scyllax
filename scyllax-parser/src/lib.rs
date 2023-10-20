//! A parser for CQL queries
//! See the source code and tests for examples of usage (for now).
pub mod comment;
pub mod common;
pub mod create_keyspace;
pub mod delete;
pub mod reserved;
pub mod select;
pub mod r#where;

use comment::parse_comment;
pub use common::{Column, Value, Variable};
use create_keyspace::CreateKeyspaceQuery;
pub use delete::DeleteQuery;
pub use r#where::{ComparisonOperator, WhereClause};
pub use select::SelectQuery;

use nom::{branch::alt, combinator::map, error::Error, multi::many0, Err, IResult};

/// Represents a query
/// ```rust
/// use scyllax_parser::*;
///
/// let query = Query::try_from("select id, name from person where id = ?");
/// assert_eq!(
///     query,
///     Ok(Query::Select(SelectQuery {
///         table: "person".to_string(),
///         columns: vec![
///             Column::Identifier("id".to_string()),
///             Column::Identifier("name".to_string()),
///         ],
///         condition: vec![
///             WhereClause {
///                 column: Column::Identifier("id".to_string()),
///                 operator: ComparisonOperator::Equal,
///                 value: Value::Variable(Variable::Placeholder),
///             },
///         ],
///         limit: None,
///     }))
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum Query {
    /// A select query
    Select(SelectQuery),
    /// A delete query
    Delete(DeleteQuery),
    /// A create keyspace query
    CreateKeyspace(CreateKeyspaceQuery),
}

/// Parse a CQL query.
pub fn parse_query(input: &str) -> IResult<&str, Query> {
    // trim whitespace
    let input = input.trim();
    // strip comments
    let (input, _) = many0(parse_comment)(input)?;
    let input = input.trim();
    println!("input: {input:#?}");

    alt((
        map(select::parse_select, Query::Select),
        map(delete::parse_delete, Query::Delete),
        map(
            create_keyspace::parse_create_keyspace,
            Query::CreateKeyspace,
        ),
    ))(input)
}

impl<'a> TryFrom<&'a str> for Query {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse_query(value)?.1)
    }
}

/// Parse a file that can contain multiple CQL queries. The queries are separated by a semicolon.
/// There may be an indeterminate number of newlines between the semicolon and the next query.
pub fn parse_query_file(input: &str) -> IResult<&str, Vec<Query>> {
    let trimmed = input.trim();

    let (input, queries) = nom::multi::separated_list1(
        nom::character::complete::multispace0,
        nom::sequence::terminated(parse_query, nom::character::complete::multispace0),
    )(trimmed)?;

    Ok((input, queries))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_query_select() {
        let query = "/* this is a comment */ select id, name, age
        from person
        where id = :id
        and name = :name
        and age > ?
        limit 10";
        println!("query: {:#?}", query);

        let query = Query::try_from(query);

        assert_eq!(
            query,
            Ok(Query::Select(SelectQuery {
                table: "person".to_string(),
                columns: vec![
                    Column::Identifier("id".to_string()),
                    Column::Identifier("name".to_string()),
                    Column::Identifier("age".to_string()),
                ],
                condition: vec![
                    WhereClause {
                        column: Column::Identifier("id".to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::NamedVariable("id".to_string())),
                    },
                    WhereClause {
                        column: Column::Identifier("name".to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::NamedVariable("name".to_string())),
                    },
                    WhereClause {
                        column: Column::Identifier("age".to_string()),
                        operator: ComparisonOperator::GreaterThan,
                        value: Value::Variable(Variable::Placeholder),
                    },
                ],
                limit: Some(Value::Number(10)),
            }))
        );
    }
}
