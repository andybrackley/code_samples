use std::num::ParseIntError;

use nom::{
    branch::alt,
    bytes::complete::{ tag, take_until },
    character::complete::multispace0,
    combinator::{ not, opt },
    error::{ FromExternalError, ParseError },
    multi::{ many1, many_till, separated_list1 },
    sequence::{ delimited, preceded, terminated },
    AsChar,
    Compare,
    FindSubstring,
    IResult,
    Input,
    Parser,
};

use crate::common::parser_types::{
    AbstractType,
    ParsedField,
    ParsedStruct,
    AliasType,
    EnumType,
    EnumValue,
    ParsedVariableType,
};
use crate::nom::parser_primitives::{ as_integer, identifier, keyword };

/// Parses a Julia style variable type declaration such as `Vector{Int64}`
fn var_type<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParsedVariableType, E> {
    let (input, name) = identifier().parse(input)?;
    let (input, generic_args) = opt(
        delimited(tag("{"), separated_list1(keyword(","), var_type), tag("}"))
    ).parse(input)?;

    Ok((input, ParsedVariableType::generic(name, generic_args.unwrap_or_default())))
}

/// Parses a Julia style alias declaration such as `const MyVector{T} = Vector{Int64}`
pub(crate) fn alias<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, AliasType, E> {
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
pub(crate) fn enum_macro<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, EnumType, E>
    where E: FromExternalError<&'a str, ParseIntError>
{
    let with_value = |input: &'a str| {
        let (input, id) = identifier().parse(input)?;
        let (input, value) = opt(preceded(keyword("="), as_integer::<i32, E>())).parse(input)?;

        Ok((input, EnumValue { name: id.to_string(), value: value }))
    };

    let as_flat = preceded(not(keyword("begin")), separated_list1(tag(" "), with_value));

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

/// Parses a Julia style abstract type declaration such as `abstract type MyType{T} end`
pub(crate) fn abstract_type<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, AbstractType, E> {
    let (input, _) = keyword("abstract").parse(input)?;
    let (input, _) = keyword("type").parse(input)?;
    let (input, abs_type) = var_type(input)?;
    let (input, _) = keyword("end").parse(input)?;

    Ok((
        input,
        AbstractType {
            struct_name: abs_type.name,
            generic_arguments: abs_type.generic_args,
        },
    ))
}

/// Parses a Julia style field declaration such as `x::Int64` or `x::Vector{Int64}`
fn field<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, ParsedField, E> {
    let (input, field) = identifier().parse(input)?;
    let (input, _) = keyword("::").parse(input)?;
    let (input, var_type) = var_type(input)?;

    Ok((
        input,
        ParsedField {
            field_name: field.to_string(),
            field_type: var_type,
        },
    ))
}

/// Parses a Julia style struct declaration such as `struct Person
///     name::String
///     age::Int64
/// end`
///
/// or
///
/// `struct Person{T}
///     data::Vector{T}
///     size::Int64
/// end`
///
/// or
///
/// `struct Person{T} <: AbstractPerson{T}
///     data::Vector{T}
///     size::Int64
/// end`
/// It will also parse `mutable struct` declarations such as `mutable struct MutablePoint <: AbstractPoint
pub(crate) fn struct_def<'a, E: ParseError<&'a str>>(
    input: &'a str
) -> IResult<&'a str, ParsedStruct, E> {
    struct StructHeader {
        mutable: bool,
        name: String,
        gen_args: Vec<Box<ParsedVariableType>>,
        inherit: Option<AbstractType>,
    }

    let struct_header = move |input: &'a str| -> IResult<&str, StructHeader, E> {
        let (input, mutable) = opt(keyword("mutable")).parse(input)?;
        let (input, struct_n) = preceded(keyword("struct"), var_type).parse(input)?;
        let (input, inherit) = opt(preceded(keyword("<:"), var_type)).parse(input)?;

        Ok((
            input,
            {
                StructHeader {
                    mutable: mutable.is_some(),
                    name: struct_n.name,
                    gen_args: struct_n.generic_args,
                    inherit: inherit.map(|t| AbstractType {
                        struct_name: t.name,
                        generic_arguments: t.generic_args,
                    }),
                }
            },
        ))
    };

    let (input, header) = struct_header(input)?;
    let (input, fields) = many_till(preceded(multispace0, field), keyword("end")).parse(input)?;

    Ok((
        input,
        ParsedStruct {
            struct_name: header.name,
            generic_arguments: header.gen_args,
            inherits_from: header.inherit,
            is_mutable: header.mutable,
            fields: fields.0,
        },
    ))
}

/// Parse a Julia style Single Line Comment such as `# This is a comment`
pub(crate) fn comment<'a, I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
    where I: Compare<&'a str> + Input + Clone + FindSubstring<&'a str>, <I as Input>::Item: AsChar
{
    preceded(tag("#"), terminated(take_until("\n"), tag("\n")))
}

/// Parse a Julia style include statement such as `include("common.jl")`
/// NOTE: This does not allow whitespace within the include statement
pub(crate) fn include<'a, I, E: ParseError<I>>() -> impl Parser<I, Output = I, Error = E>
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
                "Container{T}",
                ParsedVariableType::generic("Container", vec![ParsedVariableType::scalar("T")]),
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
            (
                "@enum Fruit begin
                    apple=1
                    orange =2
                    banana= 3
                    grapefruit = 123
                    none=-1
                end",
                EnumType {
                    name: "Fruit".to_string(),
                    values: vec![
                        EnumValue { name: "apple".to_string(), value: Some(1) },
                        EnumValue { name: "orange".to_string(), value: Some(2) },
                        EnumValue { name: "banana".to_string(), value: Some(3) },
                        EnumValue { name: "grapefruit".to_string(), value: Some(123) },
                        EnumValue { name: "none".to_string(), value: Some(-1) }
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

    #[test]
    fn test_abstract_type() {
        let test_cases = vec![
            (
                "abstract type MyType end",
                AbstractType {
                    struct_name: "MyType".to_string(),
                    generic_arguments: vec![],
                },
            ),
            (
                "abstract type Container{T} end",
                AbstractType {
                    struct_name: "Container".to_string(),
                    generic_arguments: vec![Box::new(ParsedVariableType::scalar("T"))],
                },
            ),
            (
                "abstract type Dict{K,V} end",
                AbstractType {
                    struct_name: "Dict".to_string(),
                    generic_arguments: vec![
                        Box::new(ParsedVariableType::scalar("K")),
                        Box::new(ParsedVariableType::scalar("V"))
                    ],
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = abstract_type::<Error<&str>>(input);
            assert!(result.is_ok(), "Failed to parse: {}, result: {}", input, result.unwrap_err());
            let (remaining, matched) = result.unwrap();
            assert_eq!(remaining, "", "Input: {}", input);
            assert_eq!(matched, expected, "Input: {}", input);
        }
    }

    #[test]
    fn test_invalid_abstract_type() {
        let invalid_cases = vec![
            "type MyType end", // missing abstract
            "abstract MyType end", // missing type keyword
            "abstract type MyType", // missing end
            "abstract type end" // missing type name
        ];

        for input in invalid_cases {
            let result = abstract_type::<Error<&str>>(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }

    #[test]
    fn test_field() {
        let test_cases = vec![
            (
                "x::Int64",
                ParsedField {
                    field_name: "x".to_string(),
                    field_type: ParsedVariableType::scalar("Int64"),
                },
            ),
            (
                "data::Vector{Float64}",
                ParsedField {
                    field_name: "data".to_string(),
                    field_type: ParsedVariableType::generic(
                        "Vector",
                        vec![ParsedVariableType::scalar("Float64")]
                    ),
                },
            ),
            (
                "dict::Dict{String,Int64}",
                ParsedField {
                    field_name: "dict".to_string(),
                    field_type: ParsedVariableType::generic(
                        "Dict",
                        vec![
                            ParsedVariableType::scalar("String"),
                            ParsedVariableType::scalar("Int64")
                        ]
                    ),
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = field::<Error<&str>>(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let (remaining, matched) = result.unwrap();
            assert_eq!(remaining, "", "Input: {}", input);
            assert_eq!(matched, expected, "Input: {}", input);
        }
    }

    #[test]
    fn test_invalid_field() {
        let invalid_cases = vec![
            "x", // missing type annotation
            "::Int64", // missing field name
            "x:Int64", // missing second colon
            "x:: " // missing type
        ];

        for input in invalid_cases {
            let result = field::<Error<&str>>(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }

    #[test]
    fn test_struct_def() {
        let test_cases = vec![
            // Empty struct
            (
                "struct EmptyStruct end",
                ParsedStruct {
                    struct_name: "EmptyStruct".to_string(),
                    generic_arguments: vec![],
                    inherits_from: None,
                    is_mutable: false,
                    fields: vec![],
                },
            ),
            // Single field struct
            (
                "struct Point
                x::Float64
            end",
                ParsedStruct {
                    struct_name: "Point".to_string(),
                    generic_arguments: vec![],
                    inherits_from: None,
                    is_mutable: false,
                    fields: vec![ParsedField {
                        field_name: "x".to_string(),
                        field_type: ParsedVariableType::scalar("Float64"),
                    }],
                },
            ),
            // Multiple fields
            (
                "struct Person
                name::String
                age::Int64
                height::Float64
            end",
                ParsedStruct {
                    struct_name: "Person".to_string(),
                    generic_arguments: vec![],
                    inherits_from: None,
                    is_mutable: false,
                    fields: vec![
                        ParsedField {
                            field_name: "name".to_string(),
                            field_type: ParsedVariableType::scalar("String"),
                        },
                        ParsedField {
                            field_name: "age".to_string(),
                            field_type: ParsedVariableType::scalar("Int64"),
                        },
                        ParsedField {
                            field_name: "height".to_string(),
                            field_type: ParsedVariableType::scalar("Float64"),
                        }
                    ],
                },
            ),
            // Generic struct
            (
                "struct Container{T}
                data::Vector{T}
                size::Int64
            end",
                ParsedStruct {
                    struct_name: "Container".to_string(),
                    generic_arguments: vec![Box::new(ParsedVariableType::scalar("T"))],
                    inherits_from: None,
                    is_mutable: false,
                    fields: vec![
                        ParsedField {
                            field_name: "data".to_string(),
                            field_type: ParsedVariableType::generic(
                                "Vector",
                                vec![ParsedVariableType::scalar("T")]
                            ),
                        },
                        ParsedField {
                            field_name: "size".to_string(),
                            field_type: ParsedVariableType::scalar("Int64"),
                        }
                    ],
                },
            ),
            // Mutable struct with inheritance
            (
                "mutable struct MutablePoint <: AbstractPoint
                x::Float64
                y::Float64
            end",
                ParsedStruct {
                    struct_name: "MutablePoint".to_string(),
                    generic_arguments: vec![],
                    inherits_from: Some(AbstractType {
                        struct_name: "AbstractPoint".to_string(),
                        generic_arguments: vec![],
                    }),
                    is_mutable: true,
                    fields: vec![
                        ParsedField {
                            field_name: "x".to_string(),
                            field_type: ParsedVariableType::scalar("Float64"),
                        },
                        ParsedField {
                            field_name: "y".to_string(),
                            field_type: ParsedVariableType::scalar("Float64"),
                        }
                    ],
                },
            ),
            // Complex generic struct with inheritance
            (
                "struct Dictionary{K,V} <: AbstractDictionary{K,V}
                keys::Vector{K}
                values::Vector{V}
                size::Int64
            end",
                ParsedStruct {
                    struct_name: "Dictionary".to_string(),
                    generic_arguments: vec![
                        Box::new(ParsedVariableType::scalar("K")),
                        Box::new(ParsedVariableType::scalar("V"))
                    ],
                    inherits_from: Some(AbstractType {
                        struct_name: "AbstractDictionary".to_string(),
                        generic_arguments: vec![
                            Box::new(ParsedVariableType::scalar("K")),
                            Box::new(ParsedVariableType::scalar("V"))
                        ],
                    }),
                    is_mutable: false,
                    fields: vec![
                        ParsedField {
                            field_name: "keys".to_string(),
                            field_type: ParsedVariableType::generic(
                                "Vector",
                                vec![ParsedVariableType::scalar("K")]
                            ),
                        },
                        ParsedField {
                            field_name: "values".to_string(),
                            field_type: ParsedVariableType::generic(
                                "Vector",
                                vec![ParsedVariableType::scalar("V")]
                            ),
                        },
                        ParsedField {
                            field_name: "size".to_string(),
                            field_type: ParsedVariableType::scalar("Int64"),
                        }
                    ],
                },
            )
        ];

        for (input, expected) in test_cases {
            let result = struct_def::<Error<&str>>(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let (remaining, matched) = result.unwrap();
            assert_eq!(remaining.trim(), "", "Input: {}", input);
            assert_eq!(
                matched,
                expected,
                "Input: {}\nExpected: {:#?}\nGot: {:#?}",
                input,
                expected,
                matched
            );
        }
    }

    #[test]
    fn test_invalid_struct_def() {
        let invalid_cases = vec![
            "struct", // missing name and body
            "struct Point", // missing end
            "mutable Point end", // missing struct keyword
            "struct Point <:", // incomplete inheritance
            "struct {T} end" // missing name
        ];

        for input in invalid_cases {
            let result = struct_def::<Error<&str>>(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }
}
