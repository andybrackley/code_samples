use super::parsed_types::*;

pub fn message_as_error(line: &str, line_number: u32, msg: &str) -> String {
    format!("Line[{line_number}-'{line}']: {msg}")
}

#[derive(Debug)]
struct Scoped {
    name: String,
    types: Vec<Box<ParsedVariableType>>,
    }
    
    impl Scoped {
        pub fn start() -> Scoped {
            Scoped {
                name: "".to_string(),
                types: Vec::new()
            }
        }
    }

pub fn parse_generic_type(type_str: &str) -> ParsedVariableType {
    fn scope_to_var(scope: &Scoped) -> ParsedVariableType {
        let name = scope.name.clone();
        let types = scope.types.clone();

        return ParsedVariableType::generic(&name, types)
    }

    fn parse_chars(chars: &[char], index: &mut usize, outer: &mut Scoped) -> ParsedVariableType {
        let mut scope = Scoped::start();

        while *index < chars.len() {
            let current = chars[*index];
            *index += 1;

            match current {
                '{' => {
                    // let mut new_scope = Scoped::start();
                    let v = parse_chars(&chars, &mut *index, &mut scope);
                    scope.types.push(Box::new(v));
                }
                '}' => {
                    let v = scope_to_var(&scope);
                    scope.types.clear();
                    scope.name.clear();
                    return v;
                },
                ',' => {
                    let current_v = scope_to_var(&scope);
                    outer.types.push(Box::new(current_v));
                    scope.name.clear();
                    scope.types.clear();
                },
                ' ' => {}
                _ => {
                    scope.name.push(current);
                }
            }
        }
        
        return scope_to_var(&scope);
    }

    let mut scope = Scoped::start(); 
    let chars: Vec<char> = type_str.chars().collect();
    let mut index = 0;

    let v = parse_chars(&chars, &mut index, &mut scope);
    return v;
}
                
pub fn parse_type_definition(field_type_str: &str) -> Result<ParsedVariableType, String> {
    let field_type = parse_generic_type(field_type_str);
    return Ok(field_type);
}

/// A field consists of a field name a separator :: and the field type,
///   i.e. 
///     is_block_trade::Bool
/// Generic types such as Vector and Optional are surrounded by {}
///   i.e.
///      trade_id::Optional{TradeId}
///      action::Union{AddOrder, EditOrder, CancelOrder}
///   Given the definition:
///      test_union::Union{Int64, TimeStamp, String}
///   This will generate a structure as follows:
///      ParsedField {
///        field_name: "test_union",
///        field_type: Generic(
///            "Union",
///            [Scaler("Int64",),Scaler("TimeStamp",),Scaler("String",),],
///        ),
///     }
pub fn parse_field(line: &str, line_number: u32) -> Result<ParsedField, String> {
    let tokens: Vec<&str> = line.trim().split("::").collect();

    match tokens.as_slice() {
        [n, t] => {
        let field_type = parse_type_definition(t);
        match field_type {
            Ok(field_type) => {
                Ok(ParsedField { field_name: n.to_string(), field_type: field_type } )
            },
            Err(msg) => Err(message_as_error(line, line_number, &msg))
        }},
        _ => Err(message_as_error(line, line_number, "Should be in format var_name::var_type"))
    }
}
