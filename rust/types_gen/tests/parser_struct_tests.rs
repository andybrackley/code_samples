#[cfg(test)]
mod parser_fields_test {
    use types_gen::{
        common::parser_types::{ AbstractType, ParsedField, ParsedStruct, ParsedVariableType },
        lexer::lexer::Lexer,
        parser::parser_struct::parse_struct_def,
    };

    fn compare(expect: &ParsedStruct, actual: &ParsedStruct) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    #[test]
    pub fn test_parse_struct_containing_no_fields() {
        let lines = r#"
struct BookUpdate
end"#;
        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expect = ParsedStruct {
            is_mutable: false,
            struct_name: "BookUpdate".to_string(),
            fields: Vec::new(),
            generic_arguments: Vec::new(),
            inherits_from: None,
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_parse_mutable_struct_containing_no_fields() {
        let lines = r#"
mutable struct BookUpdate
end"#;
        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expect = ParsedStruct {
            is_mutable: true,
            struct_name: "BookUpdate".to_string(),
            fields: Vec::new(),
            generic_arguments: Vec::new(),
            inherits_from: None,
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_parse_struct_containing_single_field() {
        let lines = r#"
mutable struct BookUpdate
    time:: Int64
end"#;

        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expected_fields = vec![ParsedField {
            field_name: "time".to_string(),
            field_type: ParsedVariableType::scalar("Int64"),
        }];

        let expected = ParsedStruct {
            is_mutable: true,
            struct_name: "BookUpdate".to_string(),
            inherits_from: None,
            generic_arguments: Vec::new(),
            fields: expected_fields,
        };

        compare(&expected, &parsed.unwrap());
    }

    #[test]
    pub fn test_parse_struct_containing_multiple_fields() {
        let lines =
            r#"
mutable struct BookUpdate
    time:: Int64
    timestamp_exch::Union{Int64, Nothing}
    instId:: Int64
    updateType:: Int64

    bids::Vector{Int64}
    asks::Vector{Int64}
end"#;

        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expected_fields = vec![
            ParsedField {
                field_name: "time".to_string(),
                field_type: ParsedVariableType::scalar("Int64"),
            },
            ParsedField {
                field_name: "timestamp_exch".to_string(),
                field_type: ParsedVariableType::generic(
                    "Union",
                    vec![
                        Box::new(ParsedVariableType::scalar("Int64")),
                        Box::new(ParsedVariableType::scalar("Nothing"))
                    ]
                ),
            },

            ParsedField {
                field_name: "instId".to_string(),
                field_type: ParsedVariableType::scalar("Int64"),
            },
            ParsedField {
                field_name: "updateType".to_string(),
                field_type: ParsedVariableType::scalar("Int64"),
            },

            ParsedField {
                field_name: "bids".to_string(),
                field_type: ParsedVariableType::generic(
                    "Vector",
                    vec![Box::new(ParsedVariableType::scalar("Int64"))]
                ),
            },

            ParsedField {
                field_name: "asks".to_string(),
                field_type: ParsedVariableType::generic(
                    "Vector",
                    vec![Box::new(ParsedVariableType::scalar("Int64"))]
                ),
            }
        ];

        let expected = ParsedStruct {
            is_mutable: true,
            struct_name: "BookUpdate".to_string(),
            inherits_from: None,
            generic_arguments: Vec::new(),
            fields: expected_fields,
        };

        compare(&expected, &parsed.unwrap());
    }

    #[test]
    pub fn test_parse_struct_with_generics() {
        let lines = r#"
mutable struct BookUpdate{A, B, C}
end"#;
        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expect = ParsedStruct {
            is_mutable: true,
            struct_name: "BookUpdate".to_string(),
            fields: Vec::new(),
            generic_arguments: vec![
                Box::new(ParsedVariableType::scalar("A")),
                Box::new(ParsedVariableType::scalar("B")),
                Box::new(ParsedVariableType::scalar("C"))
            ],
            inherits_from: None,
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_parse_struct_with_generics_and_inherits() {
        let lines = r#"
mutable struct BookUpdate{A, B, C} <: Union{D, E, F}
end"#;
        let lex = Lexer::parse(lines);
        let parsed = parse_struct_def(&lex.tokens, &mut 0);

        let expect = ParsedStruct {
            is_mutable: true,
            struct_name: "BookUpdate".to_string(),
            fields: Vec::new(),
            generic_arguments: vec![
                Box::new(ParsedVariableType::scalar("A")),
                Box::new(ParsedVariableType::scalar("B")),
                Box::new(ParsedVariableType::scalar("C"))
            ],
            inherits_from: Some(AbstractType {
                struct_name: "Union".to_string(),
                generic_arguments: vec![
                    Box::new(ParsedVariableType::scalar("D")),
                    Box::new(ParsedVariableType::scalar("E")),
                    Box::new(ParsedVariableType::scalar("F"))
                ],
            }),
        };

        compare(&expect, &parsed.unwrap());
    }
}
