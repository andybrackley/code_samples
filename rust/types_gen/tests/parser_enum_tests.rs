#[cfg(test)]
pub mod parser_enum_tests {
    use types_gen::{
        common::parser_types::{ EnumType, EnumValue },
        lexer::lexer::Lexer,
        parser::parser_enum::parse_enum,
    };

    fn compare(expect: &EnumType, actual: &EnumType) {
        assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    }

    #[test]
    pub fn test_single_line_enum_x() {
        let line = "@enum Fruit apple";
        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);

        let expect = EnumType {
            name: "Fruit".to_string(),
            values: vec![EnumValue { name: "apple".to_string(), value: None }],
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_single_line_enum() {
        let line = "@enum Fruit apple orange banana";
        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);

        let expect = EnumType {
            name: "Fruit".to_string(),
            values: vec![
                EnumValue { name: "apple".to_string(), value: None },
                EnumValue { name: "orange".to_string(), value: None },
                EnumValue { name: "banana".to_string(), value: None }
            ],
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_single_line_enum_with_equality() {
        let line = "@enum Color red=1 blue=2 green=3";
        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);

        let expect = EnumType {
            name: "Color".to_string(),
            values: vec![
                EnumValue { name: "red".to_string(), value: Some(1) },
                EnumValue { name: "blue".to_string(), value: Some(2) },
                EnumValue { name: "green".to_string(), value: Some(3) }
            ],
        };

        compare(&expect, &parsed.unwrap());
    }

    #[test]
    pub fn test_multiline_enum() {
        let line = r#"@enum Status begin
    active=1
    inactive=0
    pending=-1
end"#;

        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);

        let expect = EnumType {
            name: "Status".to_string(),
            values: vec![
                EnumValue { name: "active".to_string(), value: Some(1) },
                EnumValue { name: "inactive".to_string(), value: Some(0) },
                EnumValue { name: "pending".to_string(), value: Some(-1) }
            ],
        };

        compare(&expect, &parsed.unwrap());
    }
}
