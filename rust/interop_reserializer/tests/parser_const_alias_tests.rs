#[cfg(test)]
pub mod parser_const_alias_tests {
    use interop_reserializer::{
        lexer::Lexer,
        parser::parser_const_alias::parse_const_alias,
        parser_types::{ AliasType, ParsedVariableType },
    };

    fn compare(expect: AliasType, actual: AliasType) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    #[test]
    pub fn test_const_alias_definition() {
        let line = "const TestAlias{T} = TestImpl2{U}";
        let lexer = Lexer::parse(line);
        let parsed = parse_const_alias(&lexer.tokens, &mut 0);

        let expect = AliasType {
            alias_type: ParsedVariableType::generic(
                "TestAlias",
                vec![Box::new(ParsedVariableType::scalar("T"))]
            ),

            target_type: ParsedVariableType::generic(
                "TestImpl2",
                vec![Box::new(ParsedVariableType::scalar("U"))]
            ),
        };

        compare(expect, parsed.unwrap());
    }
}
