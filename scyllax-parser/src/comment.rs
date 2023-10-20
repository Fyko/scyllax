use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::delimited,
    IResult,
};

// matches a cql comment
// - `-- end of line comment`
// - `/* block comment */` (can be multiline)
// - `// end of line comment`
pub fn parse_comment(input: &str) -> IResult<&str, &str> {
    alt((
        parse_line_comment,
        parse_block_comment,
        parse_line_comment_slash_slash,
    ))(input)
}

fn parse_line_comment(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("--"),
        recognize(many0_count(alt((alpha1, alphanumeric1, tag(" "))))),
        tag("\n"),
    )(input)
}

fn parse_block_comment(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("/*"),
        recognize(many0_count(alt((alpha1, alphanumeric1, tag(" "))))),
        tag("*/"),
    )(input)
}

fn parse_line_comment_slash_slash(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("//"),
        recognize(many0_count(alt((alpha1, alphanumeric1, tag(" "))))),
        tag("\n"),
    )(input)
}
