use crate::{lexer_types::{Token, TokenType}, parser_types::{AbstractType, ParsedStruct}};
use super::{parser_field::parse_field_defs, parser_variable_type::parse_variable_type};

type ErrorT = String;
type ReturnT = Result<ParsedStruct, ErrorT>;

fn parse_struct_header(tokens: &Vec<Token>, token_pos: &mut usize) -> Result<ParsedStruct, ErrorT> {
    let is_mutable = *token_pos > 0 && matches!(tokens[*token_pos - 1].token_type, TokenType::Mutable);
    
    *token_pos += 1; // Skip TokenType::Struct

    let var_type = parse_variable_type(tokens, token_pos);
  
    let next_token = &tokens.get(*token_pos).map(|t|&t.token_type);
  
    let inherit_type =
        match next_token {
            Some(TokenType::InheritSymbol) => {
                *token_pos += 1;
                 let res = parse_variable_type(tokens, token_pos);
                 match res {
                    Ok(vt) => {
                        let abs = AbstractType {
                            struct_name: vt.name,
                            generic_arguments: vt.generic_args
                        };
                        Some(abs)
                    }
                    Err(e) => return Err(e)
                 }
            },
            _ => {
                None
            }
        };

    let header = 
        match var_type {
            Ok(vt) => {
                let st = ParsedStruct {
                    struct_name: vt.name,
                    generic_arguments: vt.generic_args,
                    is_mutable,
                    fields: Vec::new(),
                    inherits_from: inherit_type
                };

                Ok(st)
            },
            Err(e) => {
                return Err(e)
            } 
        };


    return header;
}

pub fn parse_struct_def(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    let mut s: Option<ParsedStruct> = None;
    let mut fields = Vec::new();
    
    while *token_pos < tokens.len() {
        let current = &tokens[*token_pos];
        *token_pos += 1;

        match &current.token_type {
            TokenType::End => {
                match s {
                    Some(s) => { 
                        let cpy = ParsedStruct { 
                            is_mutable: s.is_mutable,
                            struct_name: s.struct_name,
                            generic_arguments: s.generic_arguments,
                            inherits_from: s.inherits_from,
                            fields: fields
                        };

                        return Ok(cpy);
                    }, 
                    None => {
                        return Err("ParsedStruct::Incomplete".to_string());
                    }
                }
            },
            TokenType::Struct => {
                *token_pos -= 1;  
                let r = parse_struct_header(tokens, token_pos);
                match r {
                    Ok(header) => {
                        s = Some(header)
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            TokenType::NewLine => {
                if !s.is_some() { continue; }

                let parsed_fields = parse_field_defs(tokens, token_pos);
                match parsed_fields {
                    Ok(f) => {
                        fields = f;
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            _ => {}
        }        
    };

    return Err("ParserStruct::End of struct not found".to_string());
}
