use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, alphanumeric1, digit1, multispace0, space0},
    combinator::opt,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{parser_function::function, parser_operator::operator, token::Token};

pub fn integer(input: &str) -> IResult<&str, Token> {
    let (input, (_, sign, i, _)) = tuple((multispace0, opt(tag("-")), digit1, multispace0))(input)?;
    let n = i.parse::<i64>().unwrap();
    let n = sign.map_or(n, |_s| -n);
    Ok((input, Token::Integer { i: n }))
}

pub fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, (_, x, x_rest, _)) = tuple((space0, alpha1, alphanumeric0, space0))(input)?;
    let value = x.to_string() + x_rest;
    Ok((input, Token::Identifier { value }))
}
pub fn subexpression(input: &str) -> IResult<&str, Token> {
    let (input, (_, e, _)) = tuple((tag("("), expression, tag(")")))(input)?;
    Ok((input, e))
}

pub fn factor(input: &str) -> IResult<&str, Token> {
    alt((integer, identifier, subexpression))(input)
}

pub fn term(input: &str) -> IResult<&str, Token> {
    let (input, (_, left, right)) =
        tuple((multispace0, factor, many0(tuple((operator, factor)))))(input)?;
    let mut elements = vec![left];
    for e in right {
        elements.push(e.1);
        elements.push(e.0);
    }
    Ok((input, Token::Term { elements }))
}

pub fn assignment(input: &str) -> IResult<&str, Token> {
    let (input, (_, id, _, expression)) = tuple((
        multispace0,
        identifier,
        delimited(space0, tag("="), space0),
        expression,
    ))(input)?;
    Ok((
        input,
        Token::Assignment {
            id: Box::new(id),
            expression: Box::new(expression),
        },
    ))
}

pub fn expression(input: &str) -> IResult<&str, Token> {
    let (input, (_, t)) = tuple((multispace0, alt((function, assignment, term))))(input)?;
    Ok((input, Token::Expression { elements: vec![t] }))
}

mod tests {
    use super::*;
    use crate::token::Token::*;
    #[test]
    fn test_parser_assignment() {
        println!("{:#?}", assignment("x=3;").unwrap(),);
    }
    #[test]
    fn test_parser_expression() {
        println!("{:#?}", expression("(3+4)+2;").unwrap());

        // println!("{:?}", expression("((3+4)+2);").unwrap().1);
        // assert_eq!(true, expression("3+4;x=3;").is_ok());
    }
}
