//! delete query
use nom::{
    bytes::complete::tag_no_case, character::complete::multispace0, error::Error, Err, IResult,
};

use crate::{
    common::parse_identifier,
    r#where::{parse_where_clause, WhereClause},
};

/// Represents a delete query
#[derive(Debug, PartialEq)]
pub struct DeleteQuery {
    /// The table being queried
    pub table: String,
    /// The conditions of the query
    pub conditions: Vec<WhereClause>,
}

impl<'a> TryFrom<&'a str> for DeleteQuery {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse_delete(value)?.1)
    }
}

/// Parses a delete query
pub fn parse_delete(input: &str) -> IResult<&str, DeleteQuery> {
    let (input, _) = tag_no_case("delete from ")(input)?;
    let (input, table) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, conditions) = parse_where_clause(input)?;

    Ok((
        input,
        DeleteQuery {
            table: table.to_string(),
            conditions,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_parse_delete() {
        let input = "delete from person where id = ?";

        let expected = DeleteQuery {
            table: "person".to_string(),
            conditions: vec![WhereClause {
                column: Column::Identifier("id".to_string()),
                operator: r#where::ComparisonOperator::Equal,
                value: Value::Variable(Variable::Placeholder),
            }],
        };

        assert_eq!(parse_delete(input), Ok(("", expected)));
    }
}
