use crate::common::parser_types::ParsedVariableType;
use crate::lexer::lexer_types::{Token, TokenType};

#[derive(Debug)]
struct Scoped {
    name: String,
    types: Vec<Box<ParsedVariableType>>,
}

impl Scoped {
    pub fn start() -> Scoped {
        Scoped {
            name: "".to_string(),
            types: Vec::new(),
        }
    }
}

type ReturnT = Result<ParsedVariableType, String>;

pub fn parse_variable_type(tokens: &Vec<Token>, token_pos: &mut usize) -> ReturnT {
    fn scope_to_var(scope: &Scoped) -> ParsedVariableType {
        if scope.types.is_empty() {
            ParsedVariableType::scalar(&scope.name)
        } else {
            ParsedVariableType::generic(&scope.name, scope.types.clone())
        }
    }

    fn parse_tokens(
        tokens: &Vec<Token>,
        token_pos: &mut usize,
        outer: &mut Scoped,
        is_open_brace: bool,
        depth: usize
    ) -> ReturnT {
        let mut scope = Scoped::start();

        while *token_pos < tokens.len() {
            let current = &tokens[*token_pos];
            *token_pos += 1;

            match &current.token_type {
                // TODO: I shouldn't really be dealing with the Inherit and End tokens here....
                | TokenType::NewLine
                | TokenType::InheritSymbol
                | TokenType::Equal
                | TokenType::End => {
                    *token_pos -= 1;
                    break;
                }
                TokenType::OpenGeneric => {
                    let v = parse_tokens(tokens, token_pos, &mut scope, true, depth + 1);
                    match v {
                        Ok(v) => {
                            scope.types.push(Box::new(v));
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                TokenType::CloseGeneric => {
                    let v = scope_to_var(&scope);
                    scope.types.clear();
                    scope.name.clear();
                    return Ok(v);
                }
                TokenType::Comma => {
                    if !is_open_brace {
                        return Err("VariableType::Unexpected Comma in FieldType".to_string());
                    }

                    let v = scope_to_var(&scope);
                    outer.types.push(Box::new(v));
                    scope.types.clear();
                    scope.name.clear();
                }
                TokenType::Identifier(n) => {
                    scope.name = n.to_string();
                }
                _ => {
                    return Err(format!("VariableType::Token: {:#?}", current));
                }
            }
        }

        let v = scope_to_var(&scope);
        return Ok(v);
    }

    let mut scope = Scoped::start();
    parse_tokens(&tokens, token_pos, &mut scope, false, 0)
}
