use crate::{lexer_types::{Token, TokenType}, parser_types::EnumType};

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

pub fn parse_enum(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    let enum_token = tokens.get(*token_pos).filter(|t|t.token_type == TokenType::Enum);
    if enum_token.is_none() {
        return Err("Enum::Missing Enum Identifier".to_string());
    }

    *token_pos += 1;
    let enum_id = tokens.get(*token_pos).map(|t|&t.token_type);
    let _enum_name = 
        match enum_id {
            Some(TokenType::Identifier(nm)) => {
                nm
            },
            _ => {
                return Err("Enum::Missing enum Name".to_string());
            }
        };

    *token_pos += 1;

    let _const_token = tokens.get(*token_pos);
    return Err("ConstAlias::Missing 'const' specifier".to_string());
}
