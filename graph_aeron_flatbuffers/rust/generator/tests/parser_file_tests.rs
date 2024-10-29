#[cfg(test)]
mod tests {
    use generator::parser::parser_file::parse_lines;

    #[test]
    pub fn parse_file_test() {
        let file = r#"
        "#;

        let as_lines: Vec<&str> = file.split("\n").collect();

        let result = parse_lines(&as_lines);
    
    
        assert!(result.is_ok(), "{:#?}", result);
    }
}