#[cfg(test)]
pub mod parser_tests {
    use types_gen::{ lexer::lexer::Lexer, parser::parser::Parser };

    #[test]
    pub fn test_parser_two_alias() {
        let julia =
            r#"
        const TestAlias1{T} = TestImpl1{T}
        const TestAlias2{T} = TestImpl2{Z}
"#;
        // TODO: The 2nd Alias gets missed.
        //       Probably to do with the VariableType parsing....

        let lexer = Lexer::parse(julia);
        let parser = Parser::parse(&lexer.tokens);

        dbg!(parser.unwrap().tokens);
    }

    #[test]
    pub fn test_parser_full() {
        let julia =
            r#"
include("common.jl")
export BookUpdate, BookUpdateFull

@enum Status begin
    active=1
    inactive=0
    pending=-1
end

abstract type Test{T} end
abstract type TestMulti{T, U} end

const TestAlias2{T} = TestImpl2{Z}

const TestAlias1{T} = TestImpl1{T}

struct BookUpdate
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

        "#;

        let lexer = Lexer::parse(julia);
        let parser = Parser::parse(&lexer.tokens);
        dbg!(parser.unwrap().tokens);
    }

    #[test]
    pub fn julia_test_file() {
        let julia =
            r#"
@enum Fruit banana = 1 apple = 2 orange = 3

@enum Status begin
    Live = 1
end

mutable struct BookUpdate
    time::Int64
    timestamp_exch::Union{Int64,Nothing}
    instId::Int64
    updateType::Int64

    bids::Vector{Int64}
    asks::Vector{Int64}
end"#;

        let lexer = Lexer::parse(julia);
        let parser = Parser::parse(&lexer.tokens);
        dbg!(parser.unwrap().tokens);
    }
}
