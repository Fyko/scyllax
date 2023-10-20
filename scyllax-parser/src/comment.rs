use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::value,
    sequence::{delimited, pair},
    IResult,
};

// matches a cql comment
// - `-- end of line comment`
// - `/* block comment */` (can be multiline)
// - `// end of line comment`
pub fn parse_comment(input: &str) -> IResult<&str, ()> {
    alt((
        parse_line_comment,
        parse_block_comment,
        parse_line_comment_slash_slash,
    ))(input)
}

fn parse_line_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(tag("--"), is_not("\n\r")),
    )(input)
}

fn parse_block_comment(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        delimited(tag("/*"), take_until("*/"), tag("*/")),
    )(input)
}

fn parse_line_comment_slash_slash(input: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(tag("//"), is_not("\n\r")),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_start_of_line_comment() {
        let input = "-- start of line comment";
        let result = parse_comment(input);
        assert_eq!(result, Ok(("", ())));
    }

    #[test]
    fn test_block_comment() {
        let input = "/* block comment */";
        let result = parse_comment(input);
        assert_eq!(result, Ok(("", ())));
    }

    #[test]
    fn test_end_of_line_comment() {
        let input = "// end of _ line comment";
        let result = parse_comment(input);
        assert_eq!(result, Ok(("", ())));
    }
}
