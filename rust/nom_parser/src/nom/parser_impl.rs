use std::num::ParseIntError;

use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{ all_consuming, map, opt },
    error::{ FromExternalError, ParseError },
    multi::many0,
    sequence::preceded,
    IResult,
    Parser,
};

use crate::{
    nom::parser_julia_types::{ abstract_type, alias, comment, enum_macro, include, struct_def },
    common::parser_types::{ ParsedItem, ParsedType },
};

fn julia_types<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParsedType, E>
    where E: FromExternalError<&'a str, ParseIntError>
{
    alt((
        map(alias, ParsedType::Alias),
        map(abstract_type, ParsedType::Abstract),
        map(enum_macro, ParsedType::Enum),
        map(struct_def, ParsedType::Struct),
    )).parse(input)
}

fn julia_types_with_comments<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, ParsedItem, E>
    where E: FromExternalError<&'a str, ParseIntError>
{
    let (input, _) = opt(comment()).parse(input)?;
    let (input, _) = opt(include()).parse(input)?;

    alt((
        map(comment(), |cmt: &str| ParsedItem::Comment(cmt.to_string())),
        map(julia_types, ParsedItem::Type),
    )).parse(input)
}

pub(crate) fn parse_all<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, Vec<ParsedType>, E>
    where E: FromExternalError<&'a str, ParseIntError>
{
    let (input, items) = all_consuming(
        many0(preceded(multispace0, julia_types_with_comments))
    ).parse(input.trim())?;

    let v: Vec<ParsedType> = items
        .into_iter()
        .filter_map(|item| if let ParsedItem::Type(t) = item { Some(t) } else { None })
        .collect();

    Ok((input, v))
}
