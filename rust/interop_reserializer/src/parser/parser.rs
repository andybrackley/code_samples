use crate::{ lexer_types::{ Token, TokenType }, parser_types::ParsedType };

use super::{
    parser_abstract_type::parse_abstract_type,
    parser_const_alias::parse_const_alias,
    parser_enum::parse_enum,
    parser_struct::parse_struct_def,
};

#[derive(Debug)]
pub struct ParserError {
    pub token: Option<Token>,
    pub error: String,
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<ParsedType>,
}

type HandlerT = fn(tokens: &Vec<Token>, pos: &mut usize) -> Option<Result<ParsedType, String>>;

fn try_handle_abstract_type(
    tokens: &Vec<Token>,
    pos: &mut usize
) -> Option<Result<ParsedType, String>> {
    let token = tokens.get(*pos);
    if
        !matches!(
            token.map(|t| &t.token_type),
            Some(TokenType::Abstract)
        )
    {
        return None;
    }

    let res = parse_abstract_type(tokens, pos);
    let pt = res.map(|ev| ParsedType::Abstract(ev));
    return Some(pt);
}

fn try_handle_alias(tokens: &Vec<Token>, pos: &mut usize) -> Option<Result<ParsedType, String>> {
    let token = tokens.get(*pos);
    if
        !matches!(
            token.map(|t| &t.token_type),
            Some(TokenType::Const)
        )
    {
        return None;
    }

    let res = parse_const_alias(tokens, pos);
    let pt = res.map(|ev| ParsedType::Alias(ev));
    return Some(pt);
}

fn try_handle_enum(tokens: &Vec<Token>, pos: &mut usize) -> Option<Result<ParsedType, String>> {
    let token = tokens.get(*pos);
    if
        !matches!(
            token.map(|t| &t.token_type),
            Some(TokenType::Enum)
        )
    {
        return None;
    }

    let res = parse_enum(tokens, pos);
    let pt = res.map(|ev| ParsedType::Enum(ev));
    return Some(pt);
}

fn try_handle_struct(tokens: &Vec<Token>, pos: &mut usize) -> Option<Result<ParsedType, String>> {
    let token = tokens.get(*pos);
    if
        !matches!(
            token.map(|t| &t.token_type),
            Some(TokenType::Mutable) | Some(TokenType::Struct)
        )
    {
        return None;
    }

    let res = parse_struct_def(tokens, pos);
    let pt = res.map(|ev| ParsedType::Struct(ev));
    return Some(pt);
}

const HANDLERS: &[HandlerT] = &[
    try_handle_enum,
    try_handle_abstract_type,
    try_handle_alias,
    try_handle_struct,
];

impl Parser {
    fn process_token(tokens: &Vec<Token>, pos: &mut usize) -> Option<Result<ParsedType, String>> {
        for &handler in HANDLERS {
            let res = handler(tokens, pos);
            if res.is_some() {
                return res;
            }
        }

        return None;
    }

    pub fn parse(tokens: &Vec<Token>) -> Result<Parser, ParserError> {
        let mut parsed_tokens = Vec::new();
        let mut token_pos = 0;

        while token_pos < tokens.len() {
            let token = &tokens[token_pos];
            token_pos += 1;

            let result = Parser::process_token(tokens, &mut token_pos);
            match result {
                Some(Ok(t)) => {
                    parsed_tokens.push(t);
                }
                Some(Err(e)) => {
                    return Err(ParserError {
                        error: e,
                        token: Some(token.clone()),
                    });
                }
                _ => {}
            }
        }

        return Ok(Parser {
            tokens: parsed_tokens,
        });
    }
}