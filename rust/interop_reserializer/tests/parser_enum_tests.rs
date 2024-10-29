#[cfg(test)]
pub mod parser_enum_tests {
    use interop_reserializer::{lexer::Lexer, parser::parser_enum::parse_enum};

    // fn compare(expect: AliasType, actual: AliasType) {
    //     assert!(expect == actual, "e: {:#?}, a: {:#?}", expect, actual);
    // }

    #[test]
    pub fn test_single_line_enum() {
        let line = "@enum Fruit apple orange banana";
        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);
        println!("{:#?}", parsed);
    }

    #[test]
    pub fn test_single_line_enum_with_equality() {
        let line = "@enum Color red=1 blue=2 green=3";
        let lexer = Lexer::parse(line);
        let parsed = parse_enum(&lexer.tokens, &mut 0);
        println!("{:#?}", parsed);
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
        println!("{:#?}", parsed);
    }
}