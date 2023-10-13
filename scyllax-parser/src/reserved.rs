use crate::common::eof;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    combinator::{map, peek},
    sequence::terminated,
    IResult,
};

/// A reserved keyword in CQL
#[derive(Debug, PartialEq)]
pub enum ReservedKeyword {
    Add,
    Aggregate,
    Allow,
    Alter,
    And,
    Any,
    Apply,
    Asc,
    Authorize,
    Batch,
    Begin,
    By,
    ColumnFamily,
    Create,
    Delete,
    Desc,
    Drop,
    EachQuorum,
    Entries,
    From,
    Full,
    Grant,
    If,
    In,
    Index,
    Inet,
    Infinity,
    Insert,
    Into,
    Keyspace,
    Keyspaces,
    Limit,
    LocalOne,
    LocalQuorum,
    Materialized,
    Modify,
    Nan,
    NoRecursive,
    Not,
    Of,
    On,
    One,
    Order,
    Partition,
    Password,
    Per,
    Primary,
    Quorum,
    Rename,
    Revoke,
    Schema,
    Select,
    Set,
    Table,
    Time,
    Three,
    To,
    Token,
    Truncate,
    Two,
    Unlogged,
    Update,
    Use,
    Using,
    Values,
    View,
    Where,
    With,
    WriteTime,
}

macro_rules! impl_keyword_parser {
    ($keyword:literal, $enum_member:ident) => {
        map(
            terminated(tag_no_case($keyword), keyword_follow_char),
            |_| ReservedKeyword::$enum_member,
        )
    };
}

fn keywords_alpha(input: &str) -> IResult<&str, ReservedKeyword> {
    alt((
        impl_keyword_parser!("ADD", Add),
        impl_keyword_parser!("AGGREGATE", Aggregate),
        impl_keyword_parser!("ALLOW", Allow),
        impl_keyword_parser!("ALTER", Alter),
        impl_keyword_parser!("AND", And),
        impl_keyword_parser!("ANY", Any),
        impl_keyword_parser!("APPLY", Apply),
        impl_keyword_parser!("ASC", Asc),
        impl_keyword_parser!("AUTHORIZE", Authorize),
        impl_keyword_parser!("BATCH", Batch),
        impl_keyword_parser!("BEGIN", Begin),
        impl_keyword_parser!("BY", By),
        impl_keyword_parser!("COLUMNFAMILY", ColumnFamily),
        impl_keyword_parser!("CREATE", Create),
        impl_keyword_parser!("DELETE", Delete),
        impl_keyword_parser!("DESC", Desc),
        impl_keyword_parser!("DROP", Drop),
        impl_keyword_parser!("EACH_QUORUM", EachQuorum),
        impl_keyword_parser!("ENTRIES", Entries),
        impl_keyword_parser!("FROM", From),
    ))(input)
}

fn keywords_bravo(input: &str) -> IResult<&str, ReservedKeyword> {
    alt((
        impl_keyword_parser!("FULL", Full),
        impl_keyword_parser!("GRANT", Grant),
        impl_keyword_parser!("IF", If),
        impl_keyword_parser!("IN", In),
        impl_keyword_parser!("INDEX", Index),
        impl_keyword_parser!("INET", Inet),
        impl_keyword_parser!("INFINITY", Infinity),
        impl_keyword_parser!("INSERT", Insert),
        impl_keyword_parser!("INTO", Into),
        impl_keyword_parser!("KEYSPACE", Keyspace),
        impl_keyword_parser!("KEYSPACES", Keyspaces),
        impl_keyword_parser!("LIMIT", Limit),
        impl_keyword_parser!("LOCAL_ONE", LocalOne),
        impl_keyword_parser!("LOCAL_QUORUM", LocalQuorum),
        impl_keyword_parser!("MATERIALIZED", Materialized),
        impl_keyword_parser!("MODIFY", Modify),
        impl_keyword_parser!("NAN", Nan),
        impl_keyword_parser!("NORECURSIVE", NoRecursive),
        impl_keyword_parser!("NOT", Not),
        impl_keyword_parser!("OF", Of),
        impl_keyword_parser!("ON", On),
    ))(input)
}

fn keywords_charlie(input: &str) -> IResult<&str, ReservedKeyword> {
    alt((
        impl_keyword_parser!("ONE", One),
        impl_keyword_parser!("ORDER", Order),
        impl_keyword_parser!("PARTITION", Partition),
        impl_keyword_parser!("PASSWORD", Password),
        impl_keyword_parser!("PER", Per),
        impl_keyword_parser!("PRIMARY", Primary),
        impl_keyword_parser!("QUORUM", Quorum),
        impl_keyword_parser!("RENAME", Rename),
        impl_keyword_parser!("REVOKE", Revoke),
        impl_keyword_parser!("SCHEMA", Schema),
        impl_keyword_parser!("SELECT", Select),
        impl_keyword_parser!("SET", Set),
        impl_keyword_parser!("TABLE", Table),
        impl_keyword_parser!("TIME", Time),
        impl_keyword_parser!("THREE", Three),
        impl_keyword_parser!("TO", To),
        impl_keyword_parser!("TOKEN", Token),
        impl_keyword_parser!("TRUNCATE", Truncate),
        impl_keyword_parser!("TWO", Two),
        impl_keyword_parser!("UNLOGGED", Unlogged),
        impl_keyword_parser!("UPDATE", Update),
    ))(input)
}

fn keywords_delta(input: &str) -> IResult<&str, ReservedKeyword> {
    alt((
        impl_keyword_parser!("USE", Use),
        impl_keyword_parser!("USING", Using),
        impl_keyword_parser!("VALUES", Values),
        impl_keyword_parser!("VIEW", View),
        impl_keyword_parser!("WHERE", Where),
        impl_keyword_parser!("WITH", With),
    ))(input)
}

/// A reserved keyword
pub fn parse_cql_keyword(input: &str) -> IResult<&str, ReservedKeyword> {
    alt((
        keywords_alpha,
        keywords_bravo,
        keywords_charlie,
        keywords_delta,
    ))(input)
}

/// A parser that will throw an error if the input matches a keyword
/// This parser will be used for named variables to prevent collisions with keywords
pub fn fail_if_keyword(input: &str) -> IResult<&str, ReservedKeyword> {
    // if the input matches a keyword, return an error
    // otherwise, return the input
    alt((
        keywords_alpha,
        keywords_bravo,
        keywords_charlie,
        keywords_delta,
    ))(input)
}

fn keyword_follow_char(i: &str) -> IResult<&str, &str> {
    peek(alt((
        tag(" "),
        tag("\n"),
        tag(";"),
        tag("("),
        tag(")"),
        tag("\t"),
        tag(","),
        tag("="),
        eof,
    )))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cql_keyword() {
        assert_eq!(parse_cql_keyword("ADD"), Ok(("", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD "), Ok((" ", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD;"), Ok((";", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD\n"), Ok(("\n", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD("), Ok(("(", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD)"), Ok((")", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD\t"), Ok(("\t", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD,"), Ok((",", ReservedKeyword::Add)));
        assert_eq!(parse_cql_keyword("ADD="), Ok(("=", ReservedKeyword::Add)));
    }

    #[test]
    fn test_fail_parse_cql_keyword() {
        assert!(parse_cql_keyword("my_variable").is_err());
    }
}
