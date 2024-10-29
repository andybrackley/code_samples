use crate::{lexer_types::{TokenType}};

pub type ParseTokensFunc = fn(token: &TokenType) -> Result<String, String>;
pub type IsCloseTokenFunc = fn(token: &TokenType) -> bool;
pub struct ParserType {
    pub open_token:  TokenType,
    pub close_token: IsCloseTokenFunc,
    pub parse_token: ParseTokensFunc,
    pub scope_handlers: Vec<ParserType>
}
pub mod parsers_types {
    use crate::lexer_types::TokenType;
    use super::ParserType;

    pub static FIELD_PARSER: ParserType = ParserType {
        open_token: TokenType::FieldSeparator,
        close_token: |tkn| matches!(tkn, TokenType::Identifier(_)) || matches!(tkn, TokenType::End),
        // parse_token: parse_generics_type
        scope_handlers: Vec::new(),
        parse_token: |tkn| {
            println!("PARSE_FIELD: {:#?}", &tkn);
            return Ok("Yep".to_string())
        }
    };

    pub static GENERIC_PARSER: ParserType = ParserType {
        open_token: TokenType::OpenGeneric,
        close_token: |tkn| matches!(tkn, TokenType::CloseGeneric),
        scope_handlers: Vec::new(),
        parse_token: |tkn| {
            println!("PARSE_GENERIC: {:#?}", &tkn);
            return Ok("Yep".to_string())
        }
    };
} 

pub struct ParserFactory {}
impl ParserFactory {
    pub fn get_token_handler(token_type: &TokenType) -> Option<&ParserType> {
        match token_type {
            TokenType::OpenGeneric => Some(&parsers_types::FIELD_PARSER),
            // TokenType::FieldSeparator => Some(parse_field_type),
            _ => None
        }
    }
    
}