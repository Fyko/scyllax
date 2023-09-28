//! common parsing functions
use nom::{
    branch::alt,
    bytes::complete::escaped_transform,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alphanumeric1, digit1, none_of, one_of},
    combinator::map,
    IResult,
};

/// Represents a column
#[derive(Debug, PartialEq)]
pub enum Column {
    /// The column being quried has a name that's a string.
    /// Note: this can include double-quote escaped identifiers.
    Identifier(String),
    /// The column being queried is an asterisk
    Asterisk,
}

/// Represents a query variable.
#[derive(Debug, PartialEq)]
pub enum Variable {
    /// The variable is a question mark
    Placeholder,
    /// The variable is a named variable
    NamedVariable(String),
}

/// Parses a [`Variable`]
pub fn parse_variable(input: &str) -> IResult<&str, Variable> {
    alt((
        map(parse_placeholder, |_| Variable::Placeholder),
        map(parse_named_variable, |ident| {
            Variable::NamedVariable(ident.to_string())
        }),
    ))(input)
}

fn parse_named_variable(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(":")(input)?;
    parse_identifier(input)
}

/// Represents a query value -- either a variable or a literal
#[derive(Debug, PartialEq)]
pub enum Value {
    /// The value is a variable
    Variable(Variable),
    /// The value is a literal
    Literal(String),
    /// The value is a number
    Number(usize),
}

/// Parses a [`Value`]
pub fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(parse_variable, Value::Variable),
        map(parse_literal, Value::Literal),
        map(parse_number, Value::Number),
    ))(input)
}

/// Parses a [`Value::Literal`]
fn parse_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("\"")(input)?;
    let (input, literal) = escaped_transform(none_of("\""), '\\', one_of("\""))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, literal))
}

/// Parses a [`Value::Number`]
fn parse_number(input: &str) -> IResult<&str, usize> {
    let (input, number) = digit1(input)?;
    Ok((input, number.parse().unwrap()))
}

/// Parses an identifier on.. idk tbd
pub fn parse_identifier(input: &str) -> IResult<&str, &str> {
    // Assuming identifiers are alphanumeric and start with an alphabet
    alphanumeric1(input)
}

/// Parses a [`Variable::Placeholder`]
fn parse_placeholder(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("?")(input)?;
    Ok((input, "?".to_string()))
}

/// Parses a limit clause
pub fn parse_limit_clause(input: &str) -> IResult<&str, Value> {
    let (input, _) = tag_no_case("limit ")(input)?;
    let (input, limit) = parse_value(input)?;

    Ok((input, limit))
}
