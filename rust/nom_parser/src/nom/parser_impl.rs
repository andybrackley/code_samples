use std::num::ParseIntError;

use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{ all_consuming, map, opt },
    error::{ FromExternalError, ParseError },
    multi::many0,
    sequence::preceded,
    AsChar,
    Compare,
    FindSubstring,
    IResult,
    Input,
    Offset,
    Parser,
};

use crate::{
    nom::parser_julia_types::{ abstract_type, alias, comment, enum_macro, include, struct_def },
    common::parser_types::{ ParsedItem, ParsedType },
};

fn julia_types<'a, I, E: ParseError<I>>(input: I) -> IResult<I, ParsedType, E>
    where
        I: Input + Offset + Clone + AsRef<str> + Compare<&'a str>,
        E: ParseError<I> + FromExternalError<I, ParseIntError>,
        <I as Input>::Item: AsChar,
        E: FromExternalError<I, ParseIntError>
{
    alt((
        map(alias, ParsedType::Alias),
        map(abstract_type, ParsedType::Abstract),
        map(enum_macro, ParsedType::Enum),
        map(struct_def, ParsedType::Struct),
    )).parse(input)
}

fn julia_types_with_comments<'a, I, E: ParseError<I>>(input: I) -> IResult<I, ParsedItem, E>
    where
        I: Input + Offset + Clone + AsRef<str> + Compare<&'a str> + FindSubstring<&'a str>,
        E: ParseError<I> + FromExternalError<I, ParseIntError>,
        <I as Input>::Item: AsChar,
        E: FromExternalError<I, ParseIntError>
{
    let (input, _) = opt(comment()).parse(input)?;
    let (input, _) = opt(include()).parse(input)?;

    alt((
        map(comment(), |cmt: I| ParsedItem::Comment(cmt.as_ref().to_string())),
        map(julia_types, ParsedItem::Type),
    )).parse(input)
}

pub(crate) fn parse_all<'a, I, E: ParseError<I>>(input: I) -> IResult<I, Vec<ParsedType>, E>
    where
        I: Input + Offset + Clone + AsRef<str> + Compare<&'a str> + FindSubstring<&'a str>,
        E: ParseError<I> + FromExternalError<I, ParseIntError>,
        <I as Input>::Item: AsChar,
        E: FromExternalError<I, ParseIntError>
{
    let (input, items) = all_consuming(
        many0(preceded(multispace0, julia_types_with_comments))
    ).parse(input)?;

    let v: Vec<ParsedType> = items
        .into_iter()
        .filter_map(|item| if let ParsedItem::Type(t) = item { Some(t) } else { None })
        .collect();

    Ok((input, v))
}
