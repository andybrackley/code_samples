use crate::lexer::lexer_types::{Token, TokenType};

#[derive(Debug)]
pub struct Parsed {
    pub tokens: Vec<Token>,
    pub line_count: u32,
}

pub struct Lexer {}

impl Lexer {
    fn check_buffer(buffer: &str) -> Option<TokenType> {
        match buffer {
            "const" => Some(TokenType::Const),
            "@enum" => Some(TokenType::Enum),
            "abstract" => Some(TokenType::Abstract),
            "mutable" => Some(TokenType::Mutable),
            "type" => Some(TokenType::Type),
            "struct" => Some(TokenType::Struct),
            "begin" => Some(TokenType::Begin),
            "end" => Some(TokenType::End),
            "::" => Some(TokenType::FieldSeparator),
            "<:" => Some(TokenType::InheritSymbol),
            "{" => Some(TokenType::OpenGeneric),
            "}" => Some(TokenType::CloseGeneric),
            "=" => Some(TokenType::Equal),
            "," => Some(TokenType::Comma),
            "\n" | "\r" => Some(TokenType::NewLine),
            "" => None,
            _ => Some(TokenType::Identifier(buffer.to_string())),
        }
    }

    fn process_buffer(
        buffer: &mut String,
        tokens: &mut Vec<Token>,
        line_number: u32,
        char_pos: u32,
        token_pos: &mut u32
    ) -> Option<TokenType> {
        match Lexer::check_buffer(&buffer) {
            Some(token) => {
                *token_pos += 1;
                buffer.clear();
                let as_token = Token {
                    token_type: token.clone(),
                    line_number: line_number,
                    char_pos: char_pos,
                    token_pos: *token_pos,
                };

                tokens.push(as_token);
                Some(token)
            }
            None => None,
        }
    }

    fn is_separator(ch: char) -> bool {
        return ch == ' ';
    }

    fn is_new_line(ch: char) -> bool {
        return ch == '\n' || ch == '\r';
    }

    pub fn parse(lines: &str) -> Parsed {
        let mut buffer = "".to_string();
        let mut tokens = Vec::new();
        let mut line_number = if lines.is_empty() { 0 } else { 1 };
        let mut char_pos = 0;
        let mut token_pos = 0;

        for ch in lines.chars() {
            char_pos += 1;
            match ch {
                ch if Lexer::is_separator(ch) => {
                    let result = Lexer::process_buffer(
                        &mut buffer,
                        &mut tokens,
                        line_number,
                        char_pos,
                        &mut token_pos
                    );

                    match result {
                        Some(_) => {}
                        None => {
                            buffer.clear();
                        }
                    }
                }
                // TODO:  Pattern match should cater for both \n and \r rather
                //        by using is_new_line in the match
                '=' | '{' | '}' | ',' | '\n' | '\r' => {
                    if Lexer::is_new_line(ch) {
                        line_number += 1;
                        char_pos = 0;
                    }

                    Lexer::process_buffer(
                        &mut buffer,
                        &mut tokens,
                        line_number,
                        char_pos,
                        &mut token_pos
                    );
                    buffer.push(ch);
                    Lexer::process_buffer(
                        &mut buffer,
                        &mut tokens,
                        line_number,
                        char_pos,
                        &mut token_pos
                    );
                }
                ':' | '<' => {
                    // This could be a field separator which consists of two ::
                    // Therefore we want to treat as a separator but also push this
                    // character to the buffer.
                    match buffer.chars().last() {
                        Some(':') | Some('<') => {
                            buffer.push(ch);
                            Lexer::process_buffer(
                                &mut buffer,
                                &mut tokens,
                                line_number,
                                char_pos,
                                &mut token_pos
                            );
                        }
                        _ => {
                            let result = Lexer::check_buffer(&buffer);
                            result.map(|t|
                                tokens.push(Token {
                                    token_type: t,
                                    line_number,
                                    char_pos,
                                    token_pos,
                                })
                            );

                            buffer.clear();
                            buffer.push(ch);
                        }
                    }
                }
                ch => {
                    buffer.push(ch);
                }
            }
        }

        Lexer::process_buffer(&mut buffer, &mut tokens, line_number, char_pos, &mut token_pos);

        line_number += 1;
        tokens.push(Token {
            token_type: TokenType::NewLine,
            line_number: line_number,
            char_pos: 0,
            token_pos,
        });

        Parsed {
            tokens: tokens,
            line_count: line_number,
        }
    }
}
