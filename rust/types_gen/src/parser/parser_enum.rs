use crate::common::parser_types::{EnumType, EnumValue};
use crate::lexer::lexer_types::{Token, TokenType};

type ErrorT = String;
type ReturnT = Result<EnumType, ErrorT>;

/*
  Enum variations:

    @enum Fruit apple orange banana
    @enum Color red=1 blue=2 green=3

    @enum Status begin
        active=1
        inactive=0
        pending=-1
    end
*/
fn parse_enum_value(tokens: &Vec<Token>, token_pos: &mut usize) -> Result<EnumValue, ErrorT> {
    let name_tok = tokens.get(*token_pos).map(|t| &t.token_type);
    let name = if let Some(TokenType::Identifier(n)) = name_tok {
        n
    } else {
        return Err("EnumValue::Unexpected Token before name identifier".to_string());
    };

    let mut is_equal_id = false;
    let mut enum_value: Option<i32> = None;

    while *token_pos < tokens.len() {
        *token_pos += 1;

        let current = &tokens[*token_pos].token_type;
        match current {
            TokenType::Equal => {
                if is_equal_id {
                    return Err("EnumValue::Unexpected 'equal' symbol found".to_string());
                }

                is_equal_id = true;
            }
            TokenType::Identifier(value) => {
                if !is_equal_id || enum_value.is_some() {
                    *token_pos -= 1;

                    let e = EnumValue {
                        name: name.to_string(),
                        value: enum_value,
                    };

                    return Ok(e);
                } else {
                    let as_int = value.parse();
                    if as_int.is_err() {
                        return Err(format!("EnumValue::Value '{value}' should be numeric"));
                    }

                    enum_value = Some(as_int.unwrap());
                }
            }
            TokenType::End | TokenType::NewLine => {
                *token_pos -= 1;

                let e = EnumValue {
                    name: name.to_string(),
                    value: enum_value,
                };

                return Ok(e);
            }
            _ => {
                *token_pos -= 1;
                match (is_equal_id, enum_value) {
                    (true, Some(v)) => {
                        return Ok(EnumValue {
                            name: name.to_string(),
                            value: Some(v),
                        });
                    }
                    (false, _) => {
                        return Ok(EnumValue {
                            name: name.to_string(),
                            value: None,
                        });
                    }
                    _ => {
                        return Err("EnumValue::Definition is malformed".to_string());
                    }
                }
            }
        }
    }

    return Err("EnumValue::Unexpected End of definition".to_string());
}

pub fn parse_enum(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    let enum_token = tokens.get(*token_pos).filter(|t| t.token_type == TokenType::Enum);
    if enum_token.is_none() {
        return Err("Enum::Missing @enum Identifier".to_string());
    }

    *token_pos += 1;
    let enum_id = tokens.get(*token_pos).map(|t| &t.token_type);
    let enum_name = match enum_id {
        Some(TokenType::Identifier(nm)) => { nm }
        _ => {
            return Err("Enum::Missing enum Name".to_string());
        }
    };

    let mut is_multi_line = false;
    let mut enum_values: Vec<EnumValue> = Vec::new();

    while *token_pos < tokens.len() {
        *token_pos += 1;

        let current = &tokens[*token_pos].token_type;
        match current {
            TokenType::Identifier(_) => {
                // *token_pos -= 1;
                match parse_enum_value(tokens, token_pos) {
                    Ok(v) => {
                        enum_values.push(v);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            TokenType::Begin => {
                is_multi_line = true;
            }
            TokenType::NewLine => {
                if !is_multi_line {
                    let e = EnumType {
                        name: enum_name.to_string(),
                        values: enum_values,
                    };

                    return Ok(e);
                }
            }
            TokenType::End => {
                let e = EnumType {
                    name: enum_name.to_string(),
                    values: enum_values,
                };

                return Ok(e);
            }
            _ => {
                return Err("Enum::Unexpected token in enum definition".to_string());
            }
        }
    }

    let _const_token = tokens.get(*token_pos);
    return Err("Enum::Unexpected end of enum".to_string());
}
