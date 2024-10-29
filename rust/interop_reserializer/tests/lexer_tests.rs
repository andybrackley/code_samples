
#[cfg(test)]
pub mod lexer_tests {
    use interop_reserializer::{lexer::Lexer, lexer_types::{Token, TokenType}};

    fn assert_tokens(expected: &Vec<TokenType>, actual: &Vec<Token>) {
        let types : Vec<TokenType> = actual.iter().map(|tok| tok.token_type.clone()).collect();
        assert!(&types == expected, "{}", format!("Mismatch between parsed, Expected: {:#?}, Actual: {:#?}", expected, types));
    }

    #[test]
    pub fn test_struct() {
        let to_parse = r#"mutable struct TestImpl{T} <: Test{T}
end"#;

        let parsed = Lexer::parse(to_parse);

        let expected = vec![
            TokenType::Mutable,
            TokenType::Struct,
            TokenType::Identifier("TestImpl".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("T".to_string()),
            TokenType::CloseGeneric,
            TokenType::InheritSymbol,
            TokenType::Identifier("Test".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("T".to_string()),
            TokenType::CloseGeneric,
            TokenType::NewLine,
            TokenType::End,
            TokenType::NewLine,
        ];

        let tokens = parsed.tokens;
        assert_tokens(&expected, &tokens); 
        assert!(parsed.line_count == 3);
    }

    #[test]
    pub fn test_alias_definition() {
        let to_parse = "const TestAlias{T} = TestImpl2{T}";
        let parsed = Lexer::parse(&to_parse);

        let expected = vec![
            TokenType::Const,
            TokenType::Identifier("TestAlias".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("T".to_string()),
            TokenType::CloseGeneric,
            TokenType::Equal,
            TokenType::Identifier("TestImpl2".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("T".to_string()),
            TokenType::CloseGeneric,
            TokenType::NewLine,
        ];

        assert_tokens(&expected, &parsed.tokens);
    }

    #[test]
    pub fn test_field_separator() {
        let field_seperator_tests = vec![ 
            "time::TimeStamp",
            "time :: TimeStamp",
            "time:: TimeStamp",
            "time ::TimeStamp"
        ];

        for to_parse in field_seperator_tests {
            let tokens = Lexer::parse(&to_parse);
            let expected = vec![
                TokenType::Identifier("time".to_string()),
                TokenType::FieldSeparator,
                TokenType::Identifier("TimeStamp".to_string()),
                TokenType::NewLine,
            ];
    
            assert_tokens(&expected, &tokens.tokens);
        }
    }

    #[test]
    pub fn test_field_with_generic_type() {
        let field_seperator_tests = vec![ 
            "timestamp_exch::Optional{TimeStamp}",
            "timestamp_exch:: Optional{TimeStamp}",
            "timestamp_exch :: Optional{TimeStamp}",
            "timestamp_exch ::Optional{TimeStamp}",
            "timestamp_exch ::Optional{ TimeStamp }",
            "timestamp_exch ::Optional {TimeStamp}",
        ];

        for to_parse in field_seperator_tests {
            let tokens = Lexer::parse(&to_parse);
            let expected = vec![
                TokenType::Identifier("timestamp_exch".to_string()),
                TokenType::FieldSeparator,
                TokenType::Identifier("Optional".to_string()),
                TokenType::OpenGeneric,
                TokenType::Identifier("TimeStamp".to_string()),
                TokenType::CloseGeneric,
                TokenType::NewLine,
            ];

            assert_tokens(&expected, &tokens.tokens);
        }
    }

    #[test]
    pub fn test_abstract_type_def() {
        let to_parse = "abstract type Test{T} end";
        let tokens = Lexer::parse(&to_parse);
        let expected = vec![
            TokenType::Abstract,
            TokenType::Type,
            TokenType::Identifier("Test".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("T".to_string()),
            TokenType::CloseGeneric,
            TokenType::End,
            TokenType::NewLine,
        ];

        assert_tokens(&expected, &tokens.tokens);
    }

    #[test]
    pub fn test_multiple_generic_args() {
        let to_parse = "Union{A, B, C, D}";
        let tokens = Lexer::parse(&to_parse);

        let expected = vec![
            TokenType::Identifier("Union".to_string()),
            TokenType::OpenGeneric,
            TokenType::Identifier("A".to_string()),
            TokenType::Comma,
            TokenType::Identifier("B".to_string()),
            TokenType::Comma,
            TokenType::Identifier("C".to_string()),
            TokenType::Comma,
            TokenType::Identifier("D".to_string()),
            TokenType::CloseGeneric,
            TokenType::NewLine,
        ];

        assert_tokens(&expected, &tokens.tokens);
    }

    #[test]
    pub fn full_lexer_test() {
        
    let to_parse = r#"
    include("common.jl")

export BookUpdate, BookUpdateFull

mutable struct BookUpdate
    time:: Timestamp
    timestamp_exch::Optional{Timestamp}
    instId:: InstrumentId
    updateType:: BookUpdateType

    bids::Vector{Level}
    asks::Vector{Level}
end


mutable struct BookUpdateFull 
    time:: Timestamp
    timestamp_exch::Optional{Timestamp}
    instId:: InstrumentId

    bids::Vector{Level}
    asks::Vector{Level}
end

abstract type Test{T} end


struct TestImpl{T} <: Test{T}

end

struct TestImpl2{Int64} <: Test{Int64}

end

const TestAlias{T} = TestImpl2{T}

@enum Fruit apple orange 

@enum Color red=1 blue=2 green=3

@enum Status begin
    active=1
    inactive=0
    pending=-1
end
    "#;

    let parsed = Lexer::parse(&to_parse);
    println!("{:#?}", parsed);

    }
}