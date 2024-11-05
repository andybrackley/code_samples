#[cfg(test)]
pub mod parser_abstract_type_tests {
    use interop_reserializer::{
        lexer::Lexer,
        parser::parser_abstract_type::parse_abstract_type,
        parser_types::{ AbstractType, ParsedVariableType },
    };

    fn compare(expect: AbstractType, actual: AbstractType) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    #[test]
    pub fn test_abstract_type_definition() {
        let line = "abstract type Test{Union{A, B}} end";
        let lexer = Lexer::parse(line);
        let parsed = parse_abstract_type(&lexer.tokens, &mut 0);

        let expect = AbstractType {
            struct_name: "Test".to_string(),
            generic_arguments: vec![
                Box::new(
                    ParsedVariableType::generic(
                        "Union",
                        vec![
                            Box::new(ParsedVariableType::scalar("A")),
                            Box::new(ParsedVariableType::scalar("B"))
                        ]
                    )
                )
            ],
        };

        compare(expect, parsed.unwrap());
    }
}
