//! Parses where clauses in CQL statements
//! ```ignore
//! where_clause: `relation` ( AND `relation` )*
//! relation: `column_name` `operator` `term`
//!         : '(' `column_name` ( ',' `column_name` )* ')' `operator` `tuple_literal`
//!         : TOKEN '(' `column_name` ( ',' `column_name` )* ')' `operator` `term`
//! ```
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::multispace1,
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::common::{
    parse_rust_flavored_variable, parse_string_escaped_rust_flavored_variable, parse_value, Column,
    Value,
};

/// Parses a where clause with the following format:
///
/// `where <identifier> <operator> <varialbe>`
/// - eg: `where id = ?`
/// - eg: `where id = :id and name = ?`
/// - eg: `where id in :ids`
/// - eg: `where id in ? and name = :name`
/// - eg: `where id > :id`
pub fn parse_where_clause(input: &str) -> IResult<&str, Vec<WhereClause>> {
    let (input, _) = tag_no_case("where ")(input)?;

    separated_list0(tag_no_case(" and "), parse_where_condition)(input)
}

/// Represents a single `where` clause on a CQL statement
#[derive(Debug, PartialEq)]
pub struct WhereClause {
    /// The column being queried
    pub column: Column,
    /// The operator being used
    pub operator: ComparisonOperator,
    /// The variable being compared
    pub value: Value,
}

/// Represents a comparison operator
#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    /// The comparison operator is `=`
    Equal,
    /// The comparison operator is `>`
    GreaterThan,
    /// The comparison operator is `<`
    LessThan,
    /// The comparison operator is `>=`
    GreaterThanOrEqual,
    /// The comparison operator is `<=`
    LessThanOrEqual,

    /// The comparison operator is `in`
    In,
    /// The comparison operator is `contains`
    Contains,
    /// The comparison operator is `contains_key`
    ContainsKey,
}

/// Parses the column in a where statement, considering it may be wrapped in quotes.
fn parse_where_column(input: &str) -> IResult<&str, String> {
    let (input, col) = alt((
        map(parse_string_escaped_rust_flavored_variable, |x| {
            format!("\"{x}\"")
        }),
        map(parse_rust_flavored_variable, |x: &str| x.to_string()),
    ))(input)?;

    Ok((input, col.clone()))
}

/// Parses a single where condition
fn parse_where_condition(input: &str) -> IResult<&str, WhereClause> {
    let (input, column) = parse_where_column(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operator) = parse_comparison_operator(input)?;
    let (input, _) = multispace1(input)?;
    let (input, value) = parse_value(input)?;

    Ok((
        input,
        WhereClause {
            column: Column::Identifier(column),
            operator,
            value,
        },
    ))
}

/// Parses a comparison operator
fn parse_comparison_operator(input: &str) -> IResult<&str, ComparisonOperator> {
    alt((
        map(tag(">="), |_| ComparisonOperator::GreaterThanOrEqual),
        map(tag("<="), |_| ComparisonOperator::LessThanOrEqual),
        map(tag("="), |_| ComparisonOperator::Equal),
        map(tag(">"), |_| ComparisonOperator::GreaterThan),
        map(tag("<"), |_| ComparisonOperator::LessThan),
        map(tag_no_case("in"), |_| ComparisonOperator::In),
        map(tag_no_case("contains key"), |_| {
            ComparisonOperator::ContainsKey
        }),
        map(tag_no_case("contains"), |_| ComparisonOperator::Contains),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_funky_casing() {
        assert_eq!(
            parse_where_clause(
                r#"where "userId" = ? and "actionOperation" = ? and "timeBucket" = ?"#
            ),
            Ok((
                "",
                vec![
                    WhereClause {
                        column: Column::Identifier(r#""userId""#.to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::Placeholder)
                    },
                    WhereClause {
                        column: Column::Identifier(r#""actionOperation""#.to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::Placeholder)
                    },
                    WhereClause {
                        column: Column::Identifier(r#""timeBucket""#.to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::Placeholder)
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_parse_single_where_clause() {
        assert_eq!(
            parse_where_clause("where id = ?"),
            Ok((
                "",
                vec![WhereClause {
                    column: Column::Identifier("id".to_string()),
                    operator: ComparisonOperator::Equal,
                    value: Value::Variable(Variable::Placeholder)
                }]
            ))
        );
    }

    #[test]
    fn test_parse_multiple_where_clauses() {
        assert_eq!(
            parse_where_clause("where id = ? and name > :name"),
            Ok((
                "",
                vec![
                    WhereClause {
                        column: Column::Identifier("id".to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::Placeholder)
                    },
                    WhereClause {
                        column: Column::Identifier("name".to_string()),
                        operator: ComparisonOperator::GreaterThan,
                        value: Value::Variable(Variable::NamedVariable("name".to_string()))
                    }
                ]
            ))
        );

        assert_eq!(
            parse_where_clause("where id = :id and name = :name and age > 10"),
            Ok((
                "",
                vec![
                    WhereClause {
                        column: Column::Identifier("id".to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::NamedVariable("id".to_string()))
                    },
                    WhereClause {
                        column: Column::Identifier("name".to_string()),
                        operator: ComparisonOperator::Equal,
                        value: Value::Variable(Variable::NamedVariable("name".to_string()))
                    },
                    WhereClause {
                        column: Column::Identifier("age".to_string()),
                        operator: ComparisonOperator::GreaterThan,
                        value: Value::Number(10)
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_parse_comparison_operator() {
        assert_eq!(
            parse_comparison_operator("="),
            Ok(("", ComparisonOperator::Equal))
        );
        assert_eq!(
            parse_comparison_operator(">"),
            Ok(("", ComparisonOperator::GreaterThan))
        );
        assert_eq!(
            parse_comparison_operator("<"),
            Ok(("", ComparisonOperator::LessThan))
        );
        assert_eq!(
            parse_comparison_operator(">="),
            Ok(("", ComparisonOperator::GreaterThanOrEqual))
        );
        assert_eq!(
            parse_comparison_operator("<="),
            Ok(("", ComparisonOperator::LessThanOrEqual))
        );
        assert_eq!(
            parse_comparison_operator("in"),
            Ok(("", ComparisonOperator::In))
        );
        assert_eq!(
            parse_comparison_operator("contains"),
            Ok(("", ComparisonOperator::Contains))
        );
        assert_eq!(
            parse_comparison_operator("contains key"),
            Ok(("", ComparisonOperator::ContainsKey))
        );
    }
}
