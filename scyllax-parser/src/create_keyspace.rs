//! Parses a create keyspace query.
//! ```ignore
//! create_keyspace_statement: CREATE KEYSPACE [ IF NOT EXISTS ] `keyspace_name` WITH `options`
//! ```
//! ## Examples
//! ```cql,ignore
//! CREATE KEYSPACE Excalibur
//! WITH replication = {'class': 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2' : 3}
//! AND durable_writes = true;
//! ```
//!
//! ```cql,ignore
//! CREATE KEYSPACE Excelsior
//! WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : 3};
//! ```

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while_m_n},
    character::complete::{alphanumeric0, char, multispace0},
    combinator::opt,
    error::Error,
    multi::separated_list0,
    sequence::delimited,
    Err, IResult,
};

use crate::{common::parse_rust_flavored_variable, r#where::parse_comparisons, Column, Value};
#[derive(Debug, PartialEq)]
pub struct CreateKeyspaceQuery {
    pub name: String,
    pub if_not_exists: bool,
    pub replication: ReplicationOption,
    pub durable_writes: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub enum ReplicationOption {
    SimpleStrategy(i32),
    NetworkTopologyStrategy(HashMap<String, i32>),
}

impl<'a> TryFrom<&'a str> for CreateKeyspaceQuery {
    type Error = Err<Error<&'a str>>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(parse_create_keyspace(value)?.1)
    }
}

pub fn parse_create_keyspace(input: &str) -> IResult<&str, CreateKeyspaceQuery> {
    let (input, _) = tag_no_case("create keyspace ")(input)?;
    let (input, if_not_exists) = parse_if_not_exists(input)?;

    let (input, name) = parse_keyspace_name(input)?;

    let (input, _) = multispace0(input)?;
    let (input, replication) = parse_replication(input)?;
    let (input, _) = multispace0(input)?;
    let (input, durable_writes) = parse_durable_writes(input)?;
    let (input, _) = opt(tag(";"))(input)?;

    Ok((
        input,
        CreateKeyspaceQuery {
            name,
            if_not_exists,
            replication,
            durable_writes,
        },
    ))
}

fn parse_if_not_exists(input: &str) -> IResult<&str, bool> {
    let (input, exists) = opt(tag_no_case("if not exists "))(input)?;
    Ok((input, exists.is_some()))
}

fn parse_keyspace_name(input: &str) -> IResult<&str, String> {
    let (input, name) = parse_rust_flavored_variable(input)?;
    Ok((input, name.to_string()))
}

fn parse_replication(input: &str) -> IResult<&str, ReplicationOption> {
    let (input, _) = tag_no_case("with replication =")(input)?;
    let (input, strategy) = parse_replication_object(input)?;

    let class = strategy.get("class").unwrap();
    match *class {
        "SimpleStrategy" => {
            let replication_factor = strategy.get("replication_factor").unwrap();
            let replication_factor = replication_factor.parse::<i32>().unwrap();
            Ok((input, ReplicationOption::SimpleStrategy(replication_factor)))
        }
        "NetworkTopologyStrategy" => {
            let mut map = HashMap::new();
            for (key, value) in strategy {
                if key == "class" {
                    continue;
                }
                let value = value.parse::<i32>().unwrap();
                map.insert(key.to_string(), value);
            }
            Ok((input, ReplicationOption::NetworkTopologyStrategy(map)))
        }
        _ => panic!("Unknown replication strategy: {}", class),
    }
}

/// parse the weird json like replication strategy
/// eg: `{'class': 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2' : 3}`
/// remember to parse the single quotes
fn parse_replication_object(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pairs) = separated_list0(tag(","), parse_replication_pair)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("}")(input)?;

    let mut map = HashMap::new();
    for (key, value) in pairs {
        map.insert(key, value);
    }

    Ok((input, map))
}

// - 'class': 'NetworkTopologyStrategy'
// - 'DC1' : 1
// - 'DC2' : 3
/// remember to parse the single quotes and colon and command whitespaces
fn parse_replication_pair(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = multispace0(input)?;

    let (input, key) = delimited(char('\''), parse_rust_flavored_variable, char('\''))(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;

    let string_value = delimited(char('\''), alphanumeric0, char('\''));
    let int_value = take_while_m_n(1, usize::MAX, char::is_numeric);

    let (input, value) = alt((string_value, int_value))(input)?;

    let (input, _) = multispace0(input)?;

    Ok((input, (key, value)))
}

fn parse_durable_writes(input: &str) -> IResult<&str, Option<bool>> {
    let (input, comparisons) = opt(parse_comparisons)(input)?;

    let durable_writes = comparisons.and_then(|x| {
        x.into_iter().find_map(|x| match x.column {
            Column::Identifier(ref name) if name == "durable_writes" => Some(match x.value {
                Value::Boolean(value) => value,
                _ => panic!("Expected a boolean value for durable_writes"),
            }),
            _ => None,
        })
    });

    Ok((input, durable_writes))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_simple_strategy() {
        assert_eq!(
            parse_create_keyspace(
                "CREATE KEYSPACE Excalibur WITH replication = { 'class' : 'SimpleStrategy', 'replication_factor' : 3  } AND durable_writes = true;"
            ),
            Ok((
                "",
                CreateKeyspaceQuery {
                    name: "Excalibur".to_string(),
                    if_not_exists: false,
                    replication: ReplicationOption::SimpleStrategy(3),
                    durable_writes: Some(true)
                }
            ))
        );
    }

    #[test]
    fn test_network_topology_strategy() {
        assert_eq!(
            parse_create_keyspace(
                r#"CREATE KEYSPACE Excelsior WITH replication = {    'class': 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2' : 3};"#
            ),
            Ok((
                "",
                CreateKeyspaceQuery {
                    name: "Excelsior".to_string(),
                    if_not_exists: false,
                    replication: ReplicationOption::NetworkTopologyStrategy(
                        vec![("DC1".to_string(), 1), ("DC2".to_string(), 3)]
                            .into_iter()
                            .collect()
                    ),
                    durable_writes: None
                }
            ))
        );
    }

    #[test]
    fn test_if_not_exists() {
        assert_eq!(
            parse_create_keyspace(
                r#"CREATE KEYSPACE IF NOT EXISTS Excelsior WITH replication = {'class': 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2' : 3};"#
            ),
            Ok((
                "",
                CreateKeyspaceQuery {
                    name: "Excelsior".to_string(),
                    if_not_exists: true,
                    replication: ReplicationOption::NetworkTopologyStrategy(
                        vec![("DC1".to_string(), 1), ("DC2".to_string(), 3)]
                            .into_iter()
                            .collect()
                    ),
                    durable_writes: None
                }
            ))
        );
    }

    #[test]
    fn test_durable_writes() {
        assert_eq!(
            parse_create_keyspace(
                r#"CREATE KEYSPACE Excelsior WITH replication = {'class': 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2' : 3} AND durable_writes = true;"#
            ),
            Ok((
                "",
                CreateKeyspaceQuery {
                    name: "Excelsior".to_string(),
                    if_not_exists: false,
                    replication: ReplicationOption::NetworkTopologyStrategy(
                        vec![("DC1".to_string(), 1), ("DC2".to_string(), 3)]
                            .into_iter()
                            .collect()
                    ),
                    durable_writes: Some(true)
                }
            ))
        );
    }

    #[test]
    fn test_parse_replication_object() {
        let res: HashMap<&str, &str> = vec![
            ("class", "NetworkTopologyStrategy"),
            ("DC1", "1"),
            ("DC2", "3"),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            parse_replication_object(
                r#"{ 'class' : 'NetworkTopologyStrategy', 'DC1' : 1, 'DC2': 3}"#
            ),
            Ok(("", res))
        );
    }
}
