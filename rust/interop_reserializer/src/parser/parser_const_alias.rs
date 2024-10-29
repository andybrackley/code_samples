use crate::{lexer_types::{Token, TokenType}, parser_types::{AliasType, ParsedVariableType}};

use super::parser_variable_type::parse_variable_type;

type ErrorT = String;
type ReturnT = Result<AliasType, ErrorT>;

/* const TestAlias{T} = TestImpl2{T} */
pub fn parse_const_alias(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    let const_token = tokens.get(*token_pos);
    if const_token.is_none() {
        return Err("ConstAlias::Missing 'const' specifier".to_string());
    }

    *token_pos += 1;
    let mut alias_id: Option<ParsedVariableType> = None;
    let mut mapped_to: Option<ParsedVariableType> = None;
    let mut has_equals = false;

    while *token_pos < tokens.len() {
        let current = &tokens[*token_pos].token_type;
        match current {
            TokenType::Identifier(_id) => {
                let vt = parse_variable_type(tokens, token_pos);
                match vt {
                    Ok(vt) => {
                        if has_equals {
                            mapped_to = Some(vt);
                        } else {
                            alias_id = Some(vt);
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            TokenType::Equal => {
                *token_pos += 1;
                has_equals = true;
            },
            TokenType::NewLine => {
                *token_pos += 1;

                match(alias_id, mapped_to, has_equals) {
                    (Some(id), Some(map), true) => {
                        let alias = AliasType {
                            alias_type: id,
                            target_type: map
                        };

                        return Ok(alias);
                    },
                    _ => {
                        return Err("AliasType::Incomplete Alias Definition".to_string());
                    }
                }
            },
            _ => {
                return Err("AliasType::Unexpected Token".to_string());
            }
        }
    };

    Err("ConstAlias::Incorrectly Terminated".to_string())
}