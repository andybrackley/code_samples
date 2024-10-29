use crate::{lexer_types::Token, parser_types::ParsedVariableType};

#[derive(Debug)]
pub struct ParserError {
    pub token: Option<Token>,
    pub error: String
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<ParsedVariableType>
}

impl Parser {
    pub fn parse(_tokens: &Vec<Token>) -> Result<ParsedVariableType, String> {
        // let mut parsed_tokens = Vec::new();
        let mut _token_pos = 0;

        // for token in tokens {
        // while token_pos < tokens.len() {
        //     let token = &tokens[token_pos];
        //     token_pos += 1;
 
        //     let result = parse_generics_type(tokens, &mut token_pos);
        //     match result {
        //         Ok(r) => {
        //             return Ok(r);
        //         }
        //         Err(err) => { }
        //     }
            
        // };
        return Err("No Tokens".to_string());
    }
}