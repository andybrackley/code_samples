#[cfg(test)]
pub mod parser_tests {
    use interop_reserializer::{ lexer::Lexer, parser::parser::Parser };

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
}
