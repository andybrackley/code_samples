use crate::common::parser_types::AbstractType;
use crate::lexer::lexer_types::{Token, TokenType};
use crate::parser::parser_variable_type::parse_variable_type;

type ErrorT = String;
type ReturnT = Result<AbstractType, ErrorT>;

/* abstract type Test{T} end */
pub fn parse_abstract_type(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    match 
        (tokens.get(*token_pos),     // abstract
         tokens.get(*token_pos + 1), // type
         tokens.get(*token_pos + 2)  // identifier
    ) {
        (Some(_), Some(_), Some(_)) => {
            *token_pos += 2;
            let vt = parse_variable_type(tokens, token_pos);
            match vt {
                Ok(vt) => {
                    let at = AbstractType {
                        struct_name: vt.name,
                        generic_arguments: vt.generic_args
                    };

                    let end_token = tokens.get(*token_pos).filter(|t|t.token_type == TokenType::End);
                    match end_token {
                        Some(_) => Ok(at),
                        None => {
                            Err("AbstractType::Missing 'end' Identifier".to_string())  
                        }
                    }
                },
                Err(e) => {
                    Err(e)
                }
            }
        }
        _ => Err("AbstractType::Not In Recognized Format".to_string())
    }    
}