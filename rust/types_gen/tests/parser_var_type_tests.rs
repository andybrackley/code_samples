#[cfg(test)]
mod parser_fields_test {
    use types_gen::{
        common::parser_types::ParsedVariableType,
        lexer::lexer::Lexer,
        parser::parser_variable_type::parse_variable_type,
    };

    fn compare(
        expected: Result<ParsedVariableType, String>,
        actual: Result<ParsedVariableType, String>
    ) {
        assert!(expected == actual, "e: {:#?}, a: {:#?}", expected, actual);
    }

    #[test]
    pub fn test_parse_scalar_type() {
        let field_type = "Int64";
        let tokens = Lexer::parse(&field_type);
        let parsed = parse_variable_type(&tokens.tokens, &mut 0);

        let expected = ParsedVariableType::scalar("Int64");
        compare(Ok(expected), parsed);
    }

    #[test]
    pub fn test_parse_simple_generic() {
        let field_type = "Vector{Int64}";
        let tokens = Lexer::parse(&field_type);
        let parsed = parse_variable_type(&tokens.tokens, &mut 0);

        let expected = ParsedVariableType::generic(
            "Vector",
            vec![Box::new(ParsedVariableType::scalar("Int64"))]
        );
        compare(Ok(expected), parsed);
    }

    #[test]
    pub fn test_parse_easy_nested_generic() {
        let field_type = "Vector{InnerVector{Int64}}";
        let tokens = Lexer::parse(&field_type);
        let parsed = parse_variable_type(&tokens.tokens, &mut 0);

        let inner = ParsedVariableType::generic(
            "InnerVector",
            vec![Box::new(ParsedVariableType::scalar("Int64"))]
        );

        let expected = ParsedVariableType::generic("Vector", vec![Box::new(inner)]);

        compare(Ok(expected), parsed);
    }

    #[test]
    pub fn test_parse_multi_generic() {
        let field_type = "Union{Int64, Float64, TimeStamp, String}";
        let tokens = Lexer::parse(&field_type);
        let parsed = parse_variable_type(&tokens.tokens, &mut 0);

        let expected_args = vec![
            Box::new(ParsedVariableType::scalar("Int64")),
            Box::new(ParsedVariableType::scalar("Float64")),
            Box::new(ParsedVariableType::scalar("TimeStamp")),
            Box::new(ParsedVariableType::scalar("String"))
        ];

        let expected = ParsedVariableType::generic("Union", expected_args);
        compare(Ok(expected), parsed);
    }

    #[test]
    pub fn test_parse_multi_generic_with_nesting() {
        let field_type = "Union{Int64, Vector{Float64}, TimeStamp, String}";
        let tokens = Lexer::parse(&field_type);
        let parsed = parse_variable_type(&tokens.tokens, &mut 0);

        let expected_args = vec![
            Box::new(ParsedVariableType::scalar("Int64")),
            Box::new(
                ParsedVariableType::generic(
                    "Vector",
                    vec![Box::new(ParsedVariableType::scalar("Float64"))]
                )
            ),
            Box::new(ParsedVariableType::scalar("TimeStamp")),
            Box::new(ParsedVariableType::scalar("String"))
        ];

        let expected = ParsedVariableType::generic("Union", expected_args);
        compare(Ok(expected), parsed);
    }
}
