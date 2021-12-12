use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1},
    combinator::opt,
    multi::{many0, many1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::{
    parser_expression::{expression, identifier},
    token::Token,
};

pub fn fn_name(input: &str) -> IResult<&str, Option<Token>> {
    let (input, (_, _, _, id, _)) =
        tuple((multispace0, tag("fn"), space1, opt(identifier), tag("(")))(input)?;
    Ok((input, id))
}

pub fn fn_args(input: &str) -> IResult<&str, Token> {
    let (input, (_, a, _)) = tuple((
        multispace0,
        many0(tuple((identifier, opt(tag(","))))),
        tag(")"),
    ))(input)?;
    let mut args = vec![]; // TODO issue 32838 cannot use .map
    for arg in a {
        args.push(arg.0.identifier())
    }
    Ok((input, Token::FunctionArgs { args }))
}

pub fn fn_body(input: &str) -> IResult<&str, Token> {
    let (input, (_, _, exs, _, _)) = tuple((
        multispace0,
        tag("{"),
        many0(terminated(expression, tag(";"))),
        multispace0,
        tag("}"),
    ))(input)?;
    Ok((input, Token::FunctionBody { expressions: exs }))
}

pub fn function(input: &str) -> IResult<&str, Token> {
    let (input, (name, args, body)) = tuple((fn_name, fn_args, fn_body))(input)?;
    Ok((
        input,
        Token::Function {
            name: name.map(|t| Box::new(t)),
            args: Box::new(args),
            body: Box::new(body),
        },
    ))
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_function() {
        println!(
            "{:#?}",
            function(
                "fn test ( x, y ) { 
            1+3*y/x;y+1;x=3;
        }
        "
            )
        );
    }
}
