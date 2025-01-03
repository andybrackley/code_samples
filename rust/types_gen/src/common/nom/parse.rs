use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{ alphanumeric1, multispace0, one_of },
    combinator::{ map_res, opt, recognize },
    error::VerboseError,
    multi::{ many0, many1, many_till, separated_list0 },
    sequence::{ delimited, preceded, terminated },
    IResult,
};

use crate::common::parser_types::{
    AbstractType,
    AliasType,
    EnumType,
    EnumValue,
    ParsedField,
    ParsedStruct,
    ParsedVariableType,
};

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn keyword<'a>(word: &'a str) -> impl Fn(&'a str) -> ParseResult<'a, &'a str> {
    move |input: &'a str| { delimited(multispace0, tag(word), multispace0)(input) }
}

fn identifier<'a>() -> impl Fn(&'a str) -> ParseResult<'a, &'a str> {
    move |input: &'a str| { preceded(multispace0, alphanumeric1)(input) }
}

fn parsed_numeric<'a, T: FromStr>() -> impl Fn(&'a str) -> ParseResult<'a, T> {
    move |input: &'a str| {
        map_res(recognize(many1(terminated(opt(tag("-")), one_of("0123456789")))), |v: &'a str| {
            v.parse::<T>()
        })(input)
    }
}

pub fn parse_alias<'a>(str: &'a str) -> ParseResult<AliasType> {
    let (input, _) = keyword("const")(str)?;
    let (input, alias_type) = parse_var_type(input)?;
    let (input, _) = keyword("=")(input)?;
    let (input, target_type) = parse_var_type(input)?;

    Ok((
        input,
        AliasType {
            alias_type: alias_type,
            target_type: target_type,
        },
    ))
}

pub fn parse_abstract_type<'a>(str: &'a str) -> ParseResult<AbstractType> {
    let (input, _) = keyword("abstract")(str)?;
    let (input, _) = keyword("type")(input)?;
    let (input, abs_type) = parse_var_type(input)?;
    let (input, _) = keyword("end")(input)?;

    Ok((
        input,
        AbstractType {
            struct_name: abs_type.name.to_string(),
            generic_arguments: abs_type.generic_args,
        },
    ))
}

pub fn parse_enum<'a>(str: &'a str) -> ParseResult<'a, EnumType> {
    let parse_e_with_val = |input: &'a str| -> ParseResult<'a, EnumValue> {
        let (input, name) = identifier()(input)?;
        let (input, value) = opt(preceded(keyword("="), parsed_numeric::<i32>()))(input)?;
        Ok((input, EnumValue { name: name.to_string(), value }))
    };

    let parse_flat = many1(parse_e_with_val);
    let parse_block = preceded(keyword("begin"), move |input| {
        let (input, (values, _)) = many_till(parse_e_with_val, keyword("end"))(input)?;
        Ok((input, values))
    });

    let (input, _) = keyword("@enum")(str)?;
    let (input, name) = identifier()(input)?;
    let (input, values) = alt((parse_block, parse_flat))(input)?;

    Ok((
        input,
        EnumType {
            name: name.to_string(),
            values: values,
        },
    ))
}

pub fn parse_var_type(str: &str) -> ParseResult<ParsedVariableType> {
    let parse_generic_args = delimited(
        tag("{"),
        separated_list0(tag(","), parse_var_type),
        tag("}")
    );

    let (input, id) = identifier()(str)?;
    let (input, generic_args) = opt(parse_generic_args)(input)?;

    let vt = ParsedVariableType::generic(
        id,
        generic_args.unwrap_or_else(|| [].to_vec())
    );

    Ok((input, vt))
}

pub fn parse_field(str: &str) -> ParseResult<ParsedField> {
    let (input, name) = identifier()(str)?;
    let (input, _) = keyword("::")(input)?;
    let (input, var_type) = parse_var_type(input)?;

    Ok((input, ParsedField { field_name: name.to_string(), field_type: var_type }))
}

pub fn parse_struct<'a>(str: &'a str) -> ParseResult<'a, ParsedStruct> {
    struct StructHeader {
        mutable: bool,
        name: String,
        gen_args: Vec<Box<ParsedVariableType>>,
        inherit: Option<AbstractType>,
    }

    let parse_struct_header = move |input: &'a str| -> ParseResult<StructHeader> {
        let (input, mutable) = opt(keyword("mutable"))(input)?;
        let (input, struct_n) = preceded(keyword("struct"), parse_var_type)(input)?;
        let (input, inherit) = opt(preceded(keyword("<:"), parse_var_type))(input)?;

        Ok((
            input,
            StructHeader {
                mutable: mutable.is_some(),
                name: struct_n.name.to_string(),
                gen_args: struct_n.generic_args,
                inherit: inherit.map(|vt| AbstractType {
                    struct_name: vt.name,
                    generic_arguments: vt.generic_args,
                }),
            },
        ))
    };

    let (input, header) = parse_struct_header(str)?;
    let (input, fields) = many0(preceded(multispace0, parse_field))(input)?;
    let (input, _) = keyword("end")(input)?;

    Ok((
        input,
        ParsedStruct {
            struct_name: header.name,
            generic_arguments: header.gen_args,
            inherits_from: header.inherit,
            is_mutable: header.mutable,
            fields,
        },
    ))
}

#[cfg(test)]
pub mod test_parsers {
    pub mod test_parse_alias {
        use crate::common::nom::parse::parse_alias;

        #[test]
        pub fn test_parse_alias() {
            let def = "const MyAlias{T, U} = MyType{T, U}";
            let (_, x) = parse_alias(def).unwrap();

            assert_eq!(x.alias_type.name, "MyAlias");
            assert_eq!(x.alias_type.generic_args.len(), 2);
            assert_eq!(x.target_type.name, "MyType");
            assert_eq!(x.target_type.generic_args.len(), 2);
        }
    }

    pub mod test_parse_abstract_type {
        use crate::common::{
            nom::parse::parse_abstract_type,
            parser_types::{ AbstractType, ParsedVariableType },
        };

        #[test]
        pub fn test_parse_abstract_type() {
            let def = "abstract type Test{Union{A, B}} end";
            let (_, x) = parse_abstract_type(def).unwrap();

            dbg!(&x);

            let expect = AbstractType {
                struct_name: "Test".to_string(),
                generic_arguments: vec![
                    Box::new(
                        ParsedVariableType::generic(
                            "Union",
                            vec![ParsedVariableType::scalar("A"), ParsedVariableType::scalar("B")]
                        )
                    )
                ],
            };

            assert_eq!(expect, x);
        }
    }

    pub mod test_parse_enum {
        use crate::{
            common::parser_types::{ EnumType, EnumValue },
            common::nom::parse::parse_enum,
        };

        #[test]
        fn test_enum_flat() {
            let e = "@enum Fruit apple orange banana";
            let (_, x) = parse_enum(e).unwrap();

            assert_eq!(x, EnumType {
                name: "Fruit".to_string(),
                values: vec![
                    EnumValue { name: "apple".to_string(), value: None },
                    EnumValue { name: "orange".to_string(), value: None },
                    EnumValue { name: "banana".to_string(), value: None }
                ],
            });
        }

        #[test]
        fn test_enum_flat_with_values() {
            let e = "@enum Fruit apple=1 orange=2 banana=3 test=123 none=-1";
            let (_, x) = parse_enum(e).unwrap();

            assert_eq!(x, EnumType {
                name: "Fruit".to_string(),
                values: vec![
                    EnumValue { name: "apple".to_string(), value: Some(1) },
                    EnumValue { name: "orange".to_string(), value: Some(2) },
                    EnumValue { name: "banana".to_string(), value: Some(3) },
                    EnumValue { name: "test".to_string(), value: Some(123) },
                    EnumValue { name: "none".to_string(), value: Some(-1) }
                ],
            });
        }

        #[test]
        fn test_enum_block() {
            let e =
                r#"
@enum Fruit begin
    apple=1
    orange =2
    banana= 3
    test = 123
    none=-1
end"#.trim();

            let (_, x) = parse_enum(e).unwrap();

            assert_eq!(x, EnumType {
                name: "Fruit".to_string(),
                values: vec![
                    EnumValue { name: "apple".to_string(), value: Some(1) },
                    EnumValue { name: "orange".to_string(), value: Some(2) },
                    EnumValue { name: "banana".to_string(), value: Some(3) },
                    EnumValue { name: "test".to_string(), value: Some(123) },
                    EnumValue { name: "none".to_string(), value: Some(-1) }
                ],
            });
        }
    }

    pub mod test_parse_var_type {
        use crate::common::{ nom::parse::parse_var_type, parser_types::ParsedVariableType };

        #[test]
        pub fn test_parse_simple_var_type() {
            assert_eq!(parse_var_type("Int64"), Ok(("", ParsedVariableType::scalar("Int64"))));
            assert_eq!(parse_var_type("Float64"), Ok(("", ParsedVariableType::scalar("Float64"))));
            assert_eq!(parse_var_type("String"), Ok(("", ParsedVariableType::scalar("String"))));
        }

        #[test]
        pub fn test_parse_generic_var_type() {
            assert_eq!(
                parse_var_type("Union{Int64}"),
                Ok((
                    "",
                    ParsedVariableType::generic("Union", vec![ParsedVariableType::scalar("Int64")]),
                ))
            );

            assert_eq!(
                parse_var_type("Vector{Union{Int64}}"),
                Ok((
                    "",
                    ParsedVariableType::generic(
                        "Vector",
                        vec![
                            ParsedVariableType::generic(
                                "Union",
                                vec![ParsedVariableType::scalar("Int64")]
                            )
                        ]
                    ),
                ))
            );

            assert_eq!(
                parse_var_type("Vector{Union{Int64, Float64, String}}"),
                Ok((
                    "",
                    ParsedVariableType::generic(
                        "Vector",
                        vec![
                            ParsedVariableType::generic(
                                "Union",
                                vec![
                                    ParsedVariableType::scalar("Int64"),
                                    ParsedVariableType::scalar("Float64"),
                                    ParsedVariableType::scalar("String")
                                ]
                            )
                        ]
                    ),
                ))
            );
        }
    }

    pub mod test_parser_fields {
        #[test]
        pub fn test_parse_field() {
            use crate::common::{
                nom::parse::parse_field,
                parser_types::{ ParsedField, ParsedVariableType },
            };

            assert_eq!(
                parse_field("name :: Int64"),
                Ok((
                    "",
                    ParsedField {
                        field_name: "name".to_string(),
                        field_type: ParsedVariableType::scalar("Int64"),
                    },
                ))
            );

            assert_eq!(
                parse_field("name::Int64"),
                Ok((
                    "",
                    ParsedField {
                        field_name: "name".to_string(),
                        field_type: ParsedVariableType::scalar("Int64"),
                    },
                ))
            );

            assert_eq!(
                parse_field("name::Vector{Union{Int64, Float64, String}}"),
                Ok((
                    "",
                    ParsedField {
                        field_name: "name".to_string(),
                        field_type: ParsedVariableType::generic(
                            "Vector",
                            vec![
                                ParsedVariableType::generic(
                                    "Union",
                                    vec![
                                        ParsedVariableType::scalar("Int64"),
                                        ParsedVariableType::scalar("Float64"),
                                        ParsedVariableType::scalar("String")
                                    ]
                                )
                            ]
                        ),
                    },
                ))
            );
        }
    }

    pub mod test_struct_parser {
        use crate::common::nom::parse::parse_struct;

        #[test]
        pub fn test_parse_struct() {
            let def =
                r#"
struct MyStruct{T, U} <: BaseStruct{T, U} 
    field1::Int64
    field2::Vector{Union{Int64, Float64, String}}
end"#.trim();

            let mut_def = format!("mutable {}", def);

            let (_, x) = parse_struct(def).unwrap();
            assert_eq!(x.is_mutable, false);
            assert_eq!(x.struct_name, "MyStruct");
            assert_eq!(
                x.generic_arguments
                    .iter()
                    .map(|a| a.name.clone())
                    .collect::<Vec<String>>(),
                vec!["T".to_string(), "U".to_string()]
            );
            let inherit = x.inherits_from.as_ref().unwrap();
            assert_eq!(inherit.struct_name, "BaseStruct");
            assert_eq!(
                inherit.generic_arguments
                    .iter()
                    .map(|a| a.name.clone())
                    .collect::<Vec<String>>(),
                vec!["T".to_string(), "U".to_string()]
            );

            assert_eq!(x.fields.len(), 2);

            let (_, x) = parse_struct(&mut_def).unwrap();
            assert_eq!(x.is_mutable, true);
            assert_eq!(x.struct_name, "MyStruct");

            dbg!(x);
        }
    }
}
