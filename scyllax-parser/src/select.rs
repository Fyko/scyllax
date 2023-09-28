//! Parse a Select query.
//! ```cql
//! select_statement: SELECT [ DISTINCT ] ( `select_clause` | '*' )
//!                 : FROM `table_name`
//!                 : [ WHERE `where_clause` ]
//!                 : [ GROUP BY `group_by_clause` ]
//!                 : [ ORDER BY `ordering_clause` ]
//!                 : [ PER PARTITION LIMIT (`integer` | `bind_marker`) ]
//!                 : [ LIMIT (`integer` | `bind_marker`) ]
//!                 : [ ALLOW FILTERING ]
//!                 : [ BYPASS CACHE ]
//!                 : [ USING TIMEOUT `timeout` ]
//! select_clause: `selector` [ AS `identifier` ] ( ',' `selector` [ AS `identifier` ] )*
//! selector: `column_name`
//!         : | CAST '(' `selector` AS `cql_type` ')'
//!         : | `function_name` '(' [ `selector` ( ',' `selector` )* ] ')'
//!         : | COUNT '(' '*' ')'
//! where_clause: `relation` ( AND `relation` )*
//! relation: `column_name` `operator` `term`
//!         : '(' `column_name` ( ',' `column_name` )* ')' `operator` `tuple_literal`
//!         : TOKEN '(' `column_name` ( ',' `column_name` )* ')' `operator` `term`
//! operator: '=' | '<' | '>' | '<=' | '>=' | IN | CONTAINS | CONTAINS KEY
//! ordering_clause: `column_name` [ ASC | DESC ] ( ',' `column_name` [ ASC | DESC ] )*
//! timeout: `duration`
//! ```
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    error::Error,
    multi::separated_list0,
    Err, IResult,
};

use crate::{
    common::{parse_identifier, parse_limit_clause},
    r#where::{parse_where_clause, WhereClause},
    Column, Value,
};

/// Represents a select query
#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    /// The table being queried
    pub table: String,
    /// The columns being queried
    pub columns: Vec<Column>,
    /// The conditions of the query
    pub condition: Vec<WhereClause>,
    /// The limit of the query
    pub limit: Option<Value>,
}

impl<'a> TryFrom<&'a str> for SelectQuery {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse_select(value)?.1)
    }
}

/// In `select id, name from person`:
/// Parse: `id, name`
/// note: allow selection of one column: `select id from person`
/// note: allow selection of all columns: `select * from person`
fn parse_select_clause(input: &str) -> IResult<&str, Vec<Column>> {
    separated_list0(
        tag(", "),
        map(parse_identifier, |ident| {
            Column::Identifier(ident.to_string())
        }),
    )(input)
}

fn parse_asterisk(input: &str) -> IResult<&str, Column> {
    let (input, _) = tag("*")(input)?;
    Ok((input, Column::Asterisk))
}

/// Parses a select query
pub fn parse_select(input: &str) -> IResult<&str, SelectQuery> {
    let (input, _) = tag_no_case("select ")(input)?;
    let (input, columns) = alt((
        map(parse_asterisk, |_| vec![Column::Asterisk]),
        map(parse_select_clause, |cols| cols),
    ))(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("from ")(input)?;
    let (input, table) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, condition) = opt(parse_where_clause)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, limit) = opt(parse_limit_clause)(input)?;

    Ok((
        input,
        SelectQuery {
            table: table.to_string(),
            columns,
            condition: condition.unwrap_or_default(),
            limit,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_asterisk() {
        assert_eq!(parse_asterisk("*"), Ok(("", Column::Asterisk)));
    }

    #[test]
    fn test_parse_select_clause() {
        assert_eq!(
            parse_select_clause("id, name"),
            Ok((
                "",
                vec![
                    Column::Identifier("id".to_string()),
                    Column::Identifier("name".to_string()),
                ]
            ))
        );
    }

    #[test]
    fn test_parse_limit_clause() {
        assert_eq!(
            parse_limit_clause("limit ?"),
            Ok(("", Value::Variable(Variable::Placeholder)))
        );
    }

    #[test]
    fn test_parse_select() {
        assert_eq!(
            parse_select("select * from users"),
            Ok((
                "",
                SelectQuery {
                    table: "users".to_string(),
                    columns: vec![Column::Asterisk],
                    condition: vec![],
                    limit: None,
                }
            ))
        );

        assert_eq!(
            parse_select("select id, name from users"),
            Ok((
                "",
                SelectQuery {
                    table: "users".to_string(),
                    columns: vec![
                        Column::Identifier("id".to_string()),
                        Column::Identifier("name".to_string()),
                    ],
                    condition: vec![],
                    limit: None,
                }
            ))
        );

        assert_eq!(
            parse_select("select id, name from users where id = ?"),
            Ok((
                "",
                SelectQuery {
                    table: "users".to_string(),
                    columns: vec![
                        Column::Identifier("id".to_string()),
                        Column::Identifier("name".to_string()),
                    ],
                    condition: vec![WhereClause {
                        column: Column::Identifier("id".to_string()),
                        operator: r#where::ComparisonOperator::Equal,
                        value: Value::Variable(Variable::Placeholder),
                    }],
                    limit: None,
                }
            ))
        );

        assert_eq!(
            parse_select("select id, name from users where id = :id limit ?"),
            Ok((
                "",
                SelectQuery {
                    table: "users".to_string(),
                    columns: vec![
                        Column::Identifier("id".to_string()),
                        Column::Identifier("name".to_string()),
                    ],
                    condition: vec![WhereClause {
                        column: Column::Identifier("id".to_string()),
                        operator: r#where::ComparisonOperator::Equal,
                        value: Value::Variable(Variable::NamedVariable("id".to_string())),
                    }],
                    limit: Some(Value::Variable(Variable::Placeholder)),
                }
            ))
        );

        assert_eq!(
            parse_select("select id, name from users where id in :id and age = ? limit ?"),
            Ok((
                "",
                SelectQuery {
                    table: "users".to_string(),
                    columns: vec![
                        Column::Identifier("id".to_string()),
                        Column::Identifier("name".to_string()),
                    ],
                    condition: vec![
                        WhereClause {
                            column: Column::Identifier("id".to_string()),
                            operator: r#where::ComparisonOperator::In,
                            value: Value::Variable(Variable::NamedVariable("id".to_string())),
                        },
                        WhereClause {
                            column: Column::Identifier("age".to_string()),
                            operator: r#where::ComparisonOperator::Equal,
                            value: Value::Variable(Variable::Placeholder),
                        }
                    ],
                    limit: Some(Value::Variable(Variable::Placeholder)),
                }
            ))
        );

        assert_eq!(
            parse_select("select id, name, age from person where id = :id and name = :name and age > ? limit 10"),
            Ok(( "", SelectQuery {
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
