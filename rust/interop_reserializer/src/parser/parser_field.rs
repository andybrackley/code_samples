use crate::{lexer_types::{Token, TokenType}, parser_types::{ParsedField, ParsedVariableType}};

use super::parser_variable_type::parse_variable_type;

type ErrorT = String;
type ReturnT = Result<ParsedField, ErrorT>;

pub fn parse_field_def(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    let mut field_name: Option<&str> = None;
    let mut field_type: Option<ParsedVariableType> = None;

    while *token_pos < tokens.len() {
        let token = &tokens[*token_pos];
        *token_pos += 1;

        match &token.token_type {
            TokenType::FieldSeparator => {
                let ft = parse_variable_type(tokens, token_pos);
                match ft {
                    Ok(f) => { 
                        field_type = Some(f);
                        break;
                    },
                    Err(e) => { return Err(e); } 
                }
            },
            TokenType::Identifier(n) => {
                field_name = Some(n);
            },
            TokenType::NewLine => {}
            _ => {
                // return Err("Field::Unexpected Token".to_string());
            }
        }
    };

    match (field_name, field_type) {
        (Some(n), Some(t)) => {
            let field = ParsedField { field_name: n.to_string(), field_type: t };
            Ok(field)
        },
        _ => {
            Err("Field Definition not completed".to_string())
        }
    } 
}

pub fn parse_field_defs(tokens: &Vec<Token>, token_pos: &mut usize) -> Result<Vec<ParsedField>, ErrorT> {
    let mut fields: Vec<ParsedField> = Vec::new();

    while *token_pos < tokens.len() {
        let current = tokens[*token_pos].token_type.clone();

        if  current == TokenType::NewLine {
            *token_pos += 1;
            continue;
        }

        if current == TokenType::End {
            break;
        }

        let field = parse_field_def(tokens, token_pos);
        match field {
            Ok(f) => { 
                fields.push(f);
             },
            Err(e) => {
                return Err(e);
            }
        }
    }

    return Ok(fields);
}