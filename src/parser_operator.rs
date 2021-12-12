use crate::{parser_helpers::ws, token::Token};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, sequence::tuple, IResult,
};

pub fn addition_operator(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, _)) = tuple((multispace0, tag("+"), multispace0))(input)?;
    Ok((input, Token::AdditionOperator))
}

pub fn subtraction_operator(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, _)) = tuple((multispace0, tag("-"), multispace0))(input)?;
    Ok((input, Token::SubtractionOperator))
}

pub fn multiplication_operator(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, _)) = tuple((multispace0, tag("*"), multispace0))(input)?;
    Ok((input, Token::MultiplicationOperator))
}

pub fn division_operator(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, _)) = tuple((multispace0, tag("/"), multispace0))(input)?;
    Ok((input, Token::DivisionOperator))
}

pub fn operator(input: &str) -> IResult<&str, Token> {
    alt((
        addition_operator,
        subtraction_operator,
        division_operator,
        multiplication_operator,
    ))(input)
}

mod tests {
    use super::*;
    #[test]
    fn test_operators() {
        assert_eq!(operator("+").unwrap().1, Token::AdditionOperator);
        assert_eq!(operator("-").unwrap().1, Token::SubtractionOperator);
        assert_eq!(operator("*").unwrap().1, Token::MultiplicationOperator);
        assert_eq!(operator("/").unwrap().1, Token::DivisionOperator);
        assert_eq!(operator("_").is_err(), true);
    }
}
