#[cfg(test)]
pub mod parser_field_tests {
    use interop_reserializer::{
        lexer::Lexer,
        parser::parser_field::{ parse_field_def, parse_field_defs },
        parser_types::{ ParsedField, ParsedVariableType },
    };

    fn compare_single(expect: ParsedField, actual: ParsedField) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    fn compare_multi(expect: &Vec<ParsedField>, actual: &Vec<ParsedField>) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    #[test]
    pub fn test_parse_simple_field_def() {
        let field_def = "TestField::Int64";
        let tokens = Lexer::parse(field_def);

        let parsed = parse_field_def(&tokens.tokens, &mut 0);

        let expect = ParsedField {
            field_name: "TestField".to_string(),
            field_type: ParsedVariableType::scalar("Int64"),
        };

        compare_single(expect, parsed.unwrap());
    }

    #[test]
    pub fn test_parse_field_with_generic_type() {
        let field_def = "TestField::Union{Int64, Float64}";
        let tokens = Lexer::parse(field_def);

        let parsed = parse_field_def(&tokens.tokens, &mut 0);

        let expect = ParsedField {
            field_name: "TestField".to_string(),
            field_type: ParsedVariableType::generic(
                "Union",
                vec![
                    Box::new(ParsedVariableType::scalar("Int64")),
                    Box::new(ParsedVariableType::scalar("Float64"))
                ]
            ),
        };

        compare_single(expect, parsed.unwrap());
    }

    #[test]
    pub fn test_multiple_fields() {
        let field_def =
            r#"TestField1::Int64
TestField2::Union{Int64, Float64}
TestField3::Float64

TestField4::TimeStamp
"#;

        let tokens = Lexer::parse(field_def);
        let parsed = parse_field_defs(&tokens.tokens, &mut 0);

        let expect = vec![
            ParsedField {
                field_name: "TestField1".to_string(),
                field_type: ParsedVariableType::scalar("Int64"),
            },
            ParsedField {
                field_name: "TestField2".to_string(),
                field_type: ParsedVariableType::generic(
                    "Union",
                    vec![
                        Box::new(ParsedVariableType::scalar("Int64")),
                        Box::new(ParsedVariableType::scalar("Float64"))
                    ]
                ),
            },
            ParsedField {
                field_name: "TestField3".to_string(),
                field_type: ParsedVariableType::scalar("Float64"),
            },
            ParsedField {
                field_name: "TestField4".to_string(),
                field_type: ParsedVariableType::scalar("TimeStamp"),
            }
        ];

        compare_multi(&expect, &parsed.unwrap());
    }
}
