//! A parser for CQL queries
//! See the source code and tests for examples of usage (for now).
pub mod common;
pub mod delete;
pub mod reserved;
pub mod select;
pub mod r#where;

pub use common::{Column, Value, Variable};
pub use delete::DeleteQuery;
pub use r#where::{ComparisonOperator, WhereClause};
pub use select::SelectQuery;

use nom::{error::Error, Err, IResult};

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
}

/// Parse a CQL query.
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_query_select() {
        let query = Query::try_from(
            "select id, name, age from person where id = :id and name = :name and age > ? limit 10",
        );

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
