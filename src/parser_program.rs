use nom::{
    bytes::complete::tag, character::complete::multispace0, multi::many1, sequence::tuple, IResult,
};

use crate::{parser_expression::expression, token::Token};

pub fn program(input: &str) -> IResult<&str, Token> {
    let (input, exs) = many1(tuple((expression, tag(";"))))(input)?;
    let mut exs_ = vec![];
    for e in exs {
        exs_.push(e.0)
    }
    Ok((input, Token::Program { expressions: exs_ }))
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_program() {}
}
