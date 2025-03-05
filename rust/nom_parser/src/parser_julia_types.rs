use std::{ num::ParseIntError, str::FromStr };

use nom_supreme::error::{ ErrorTree, GenericErrorTree };
use nom_supreme::ParserExt;

use nom::{
    branch::alt,
    bytes::complete::{ tag, take_till, take_until },
    character::complete::{ digit1, multispace0 },
    combinator::{ map_res, not, opt },
    error::{ FromExternalError, ParseError },
    multi::{ many1, many_till, separated_list0, separated_list1 },
    sequence::{ delimited, preceded, terminated },
    AsChar,
    Compare,
    FindSubstring,
    IResult,
    Input,
    Parser,
};

use crate::{
    parser_primitives::{ as_integer, identifier, keyword },
    types::{ AliasType, EnumType, EnumValue, ParsedVariableType },
};

/// Parse a Julia style Single Line Comment such as `# This is a comment`
pub fn comment<'a, I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
    where I: Compare<&'a str> + Input + Clone + FindSubstring<&'a str>, <I as Input>::Item: AsChar
{
    preceded(tag("#"), terminated(take_until("\n"), tag("\n")))
}

/// Parse a Julia style include statement such as `include("common.jl")`
/// NOTE: This does not allow whitespace within the include statement
pub fn include<'a, I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
    where I: Compare<&'a str> + Input + Clone + FindSubstring<&'a str>, <I as Input>::Item: AsChar
{
    preceded(
        keyword("include"),
        delimited(
            keyword("(\""),
            take_until("\""), // capture everything until the closing quote
            keyword("\")")
        )
    )
}
/// Parses a Julia style variable type declaration such as `Vector{Int64}`
pub fn var_type<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, ParsedVariableType, E> {
    let (input, name) = identifier().parse(input)?;
    let (input, generic_args) = opt(
        delimited(tag("{"), separated_list0(keyword(","), var_type), tag("}"))
    ).parse(input)?;

    Ok((input, ParsedVariableType::generic(name, generic_args.unwrap_or_default())))
}

/// Parses a Julia style alias declaration such as `const MyVector{T} = Vector{Int64}`
pub fn alias<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AliasType, E> {
    let (input, _) = keyword("const").parse(input)?;
    let (input, alias_type) = var_type(input)?;
    let (input, _) = keyword("=").parse(input)?;
    let (input, target_type) = var_type(input)?;

    Ok((
        input,
        AliasType {
            alias_type,
            target_type,
        },
    ))
}

/// Parses a Julia style enum declarations
/// `@enum Colors = Red Green Blue`
/// `@enum Colors = Red = 1 Green = 2 Blue = 3`
/// `@enum Colors begin
///     Red = 1
///     Green = 2
///     Blue = 3
/// end
pub fn enum_macro<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, EnumType, E>
    where E: FromExternalError<&'a str, ParseIntError>
{
    let with_value = |input: &'a str| {
        let (input, id) = identifier().parse(input)?;
        let (input, value) = opt(preceded(keyword("="), as_integer::<i32, E>())).parse(input)?;

        Ok((input, EnumValue { name: id.to_string(), value: value }))
    };

    let as_flat = preceded(not(keyword("begin")), many1(with_value));

    let as_block = preceded(keyword("begin"), move |input| {
        let (input, (values, _)) = many_till(with_value, keyword("end")).parse(input)?;
        Ok((input, values))
    });

    let (input, _) = keyword("@enum").parse(input)?;
    let (input, name) = identifier().parse(input)?;
    let (input, values) = alt((as_block, as_flat)).parse(input)?;

    Ok((
        input,
        EnumType {
            name: name.to_string(),
            values,
        },
    ))
}

#[cfg(test)]
pub mod test_parser_julia_types {
    use nom::{ combinator::all_consuming, error::Error };

    use super::*;

    #[test]
    fn test_comment() {
        let input = "# This is a comment\n456";
        let result = comment::<&str, Error<&str>>().parse(input);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, " This is a comment");
        assert_eq!(remaining, "456");
    }

    #[test]
    fn test_include() {
        let input = "include(\"common.jl\")";
        let result = include::<&str, nom::error::Error<&str>>().parse(input);
        assert!(result.is_ok(), "{}", result.unwrap_err());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "common.jl");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_valid_var_type() {
        let inputs = vec![
            ("Int64", ParsedVariableType::scalar("Int64")),
            (
                "Vector{Int64}",
                ParsedVariableType::generic("Vector", vec![ParsedVariableType::scalar("Int64")]),
            ),
            (
                "Union{Int64,Float64}",
                ParsedVariableType::generic(
                    "Union",
                    vec![ParsedVariableType::scalar("Int64"), ParsedVariableType::scalar("Float64")]
                ),
            ),
            (
                "Dict{String, Int64}",
                ParsedVariableType::generic(
                    "Dict",
                    vec![ParsedVariableType::scalar("String"), ParsedVariableType::scalar("Int64")]
                ),
            ),
            (
                "Union{Int64, Vector{Float64}, Dict{String, Float64}}",
                ParsedVariableType::generic(
                    "Union",
                    vec![
                        ParsedVariableType::scalar("Int64"),
                        ParsedVariableType::generic(
                            "Vector",
                            vec![ParsedVariableType::scalar("Float64")]
                        ),
                        ParsedVariableType::generic(
                            "Dict",
                            vec![
                                ParsedVariableType::scalar("String"),
                                ParsedVariableType::scalar("Float64")
                            ]
                        )
                    ]
                ),
            )
        ];

        for (input, expect) in inputs {
            let result = var_type::<Error<&str>>(input);
            assert!(result.is_ok(), "{}", result.unwrap_err());
            let (remaining, matched) = result.unwrap();

            assert_eq!(remaining, "");

            assert_eq!(
                expect,
                matched,
                "case: {}, expect: {:#?}, matched: {:#?}",
                input,
                expect,
                matched
            );
        }
    }

    #[test]
    fn test_alias() {
        let test_cases = vec![
            (
                "const MyInt = Int64",
                AliasType {
                    alias_type: ParsedVariableType::scalar("MyInt"),
                    target_type: ParsedVariableType::scalar("Int64"),
                },
            ),
            (
                "const MyVec = Vector{Int64}",
                AliasType {
                    alias_type: ParsedVariableType::scalar("MyVec"),
                    target_type: ParsedVariableType::generic(
                        "Vector",
                        vec![ParsedVariableType::scalar("Int64")]
                    ),
                },
            ),
            (
                "const MyDict = Dict{String, Int64}",
                AliasType {
                    alias_type: ParsedVariableType::scalar("MyDict"),
                    target_type: ParsedVariableType::generic(
                        "Dict",
                        vec![
                            ParsedVariableType::scalar("String"),
                            ParsedVariableType::scalar("Int64")
                        ]
                    ),
                },
            ),
            (
                "const MyComplexType = Union{Int64, Vector{Float64}}",
                AliasType {
                    alias_type: ParsedVariableType::scalar("MyComplexType"),
                    target_type: ParsedVariableType::generic(
                        "Union",
                        vec![
                            ParsedVariableType::scalar("Int64"),
                            ParsedVariableType::generic(
                                "Vector",
                                vec![ParsedVariableType::scalar("Float64")]
                            )
                        ]
                    ),
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = alias::<Error<&str>>(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let (remaining, matched) = result.unwrap();
            assert_eq!(remaining, "", "Input: {}", input);
            assert_eq!(matched, expected, "Input: {}", input);
        }
    }

    #[test]
    fn test_invalid_alias() {
        let invalid_cases = vec![
            "MyInt = Int64", // missing const
            "const = Int64", // missing alias name
            "const MyInt Int64", // missing =
            "const MyInt =" // missing target type
        ];

        for input in invalid_cases {
            let result = alias::<Error<&str>>(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }

    #[test]
    fn test_enum_macro() {
        let test_cases = vec![
            // Simple flat enum
            (
                "@enum Colors Red Green Blue",
                EnumType {
                    name: "Colors".to_string(),
                    values: vec![
                        EnumValue { name: "Red".to_string(), value: None },
                        EnumValue { name: "Green".to_string(), value: None },
                        EnumValue { name: "Blue".to_string(), value: None }
                    ],
                },
            ),
            // Flat enum with values
            (
                "@enum Numbers Red = 1 Green = 2 Blue = 3",
                EnumType {
                    name: "Numbers".to_string(),
                    values: vec![
                        EnumValue { name: "Red".to_string(), value: Some(1) },
                        EnumValue { name: "Green".to_string(), value: Some(2) },
                        EnumValue { name: "Blue".to_string(), value: Some(3) }
                    ],
                },
            ),
            // Block syntax
            (
                "@enum Colors begin
                    Red
                    Green
                    Blue
                end",
                EnumType {
                    name: "Colors".to_string(),
                    values: vec![
                        EnumValue { name: "Red".to_string(), value: None },
                        EnumValue { name: "Green".to_string(), value: None },
                        EnumValue { name: "Blue".to_string(), value: None }
                    ],
                },
            ),
            // Block syntax with values
            (
                "@enum Numbers begin
                    Red = 1
                    Green = 2
                    Blue = 3
                end",
                EnumType {
                    name: "Numbers".to_string(),
                    values: vec![
                        EnumValue { name: "Red".to_string(), value: Some(1) },
                        EnumValue { name: "Green".to_string(), value: Some(2) },
                        EnumValue { name: "Blue".to_string(), value: Some(3) }
                    ],
                },
            ),
            // Mixed values
            (
                "@enum Mixed Red = 1 Green Blue = 3",
                EnumType {
                    name: "Mixed".to_string(),
                    values: vec![
                        EnumValue { name: "Red".to_string(), value: Some(1) },
                        EnumValue { name: "Green".to_string(), value: None },
                        EnumValue { name: "Blue".to_string(), value: Some(3) }
                    ],
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = enum_macro::<Error<&str>>(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let (remaining, matched) = result.unwrap();
            assert_eq!(remaining.trim(), "", "Input: {}", input);
            assert_eq!(matched, expected, "Input: {}", input);
        }
    }

    #[test]
    fn test_invalid_enum_macro() {
        let invalid_cases = vec![
            "enum Colors Red Green Blue", // missing @
            "@enum", // missing name and values
            "@enum Colors", // missing values
            "@enum Colors begin", // unclosed block
            "@enum Colors Red = 1-.5", // non-integer value
            "@enum Colors Red = 1.5" // non-integer value
        ];

        // NOTE:
        //    We need the all_consuming to get the failures for the 1.5 case
        //    Otherwise the parser will succeed and we'll be left with the .5 remaining
        for input in invalid_cases {
            let result = all_consuming(enum_macro::<Error<&str>>).parse(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }
}
