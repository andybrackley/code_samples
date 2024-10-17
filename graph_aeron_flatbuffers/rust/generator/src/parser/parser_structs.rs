use crate::parser::parser_types::parse_field;

use super::{parsed_types::{AbstractType, AliasType, ParsedStruct}, parser_types::parse_type_definition, string_utils::split_preserving_braces};


pub fn parse_alias_part(line: &str) -> Result<AliasType, String> {
    let tokenized: Vec<&str> = split_preserving_braces(line);

    let alias_id_pos = tokenized.iter().position(|&token| token == "const");
    let tokenized_len = tokenized.len();

    match alias_id_pos {
        Some(alias_id_pos) => {
            let mut next_token_pos = alias_id_pos + 1;
            if next_token_pos >= tokenized_len { return Err("alias type definition malformed".to_string()); }

            let type_name = tokenized[next_token_pos];
            let as_type_result = parse_type_definition(type_name);

            if as_type_result.is_err() {
                return Err(format!("'abstract type' definition is malformed: {}: {line}", as_type_result.unwrap_err() ))
            }

            // TODO: The "=" might not be separated by whitespace
            next_token_pos += 1;
            if next_token_pos >= tokenized_len || tokenized[next_token_pos] != "=" { 
                { return Err("alias type definition malformed".to_string()); }
            }

            next_token_pos += 1;
            if next_token_pos >= tokenized_len { 
                { return Err("alias type definition malformed".to_string()); }
            }

            let target_type_name = tokenized[next_token_pos];
            let target_type_result = parse_type_definition(target_type_name);
            if target_type_result.is_err() {
                return Err(format!("'abstract type' definition is malformed: {}: {line}", target_type_result.unwrap_err() ))
            }
            let as_type = as_type_result.unwrap();
            let target_type = target_type_result.unwrap();

            return Ok(AliasType {
                alias_type: as_type,
                target_type: target_type
            });
        },
        _ => {
            return Err("Abstract Type Not Defined".to_string());
        }
    }
}

pub fn parse_abstract_type_part(line: &str) -> Result<AbstractType, String> {
    let tokenized: Vec<&str> = split_preserving_braces(line);

    let abstract_id_pos = tokenized.iter().position(|&token| token == "abstract");
    let tokenized_len = tokenized.len();

    let type_id_pos = tokenized.iter().position(|&token| token == "type");
    match (abstract_id_pos, type_id_pos) {
        (Some(abs_pos), Some(type_pos)) => {
            if type_pos < abs_pos { return Err("abstract type definition malformed".to_string()); }
            
            let next_token_pos = type_pos + 1;
            if next_token_pos >= tokenized_len { return Err("abstract type definition malformed".to_string()); }

            let type_name = tokenized[next_token_pos];
            let as_type_result = parse_type_definition(type_name);

            if as_type_result.is_err() {
                return Err(format!("'abstract type' definition is malformed: {}: {line}", as_type_result.unwrap_err() ))
            }

            let as_type = as_type_result.unwrap();
            return Ok(AbstractType {
                struct_name: as_type.name,
                generic_arguments: as_type.generic_args
            });
        },
        _ => {
            return Err("Abstract Type Not Defined".to_string());
        }
    }
}

/// struct Person
/// mutable struct Person
/// mutable struct Person <: AbstractPerson
pub fn parse_struct_part(line: &str) -> Result<ParsedStruct, String> {
    // let tokenized: Vec<&str> = line.split_whitespace().collect();
    let tokenized: Vec<&str> = split_preserving_braces(line);
    let struct_id_pos = tokenized.iter().position(|&token| token == "struct");
    let tokenized_len = tokenized.len();
    
    match struct_id_pos {
        Some(pos) if pos < tokenized_len => {
            let is_mutable = pos != 0 && tokenized[pos - 1] == "mutable";
            
            let mut parsed_pos = pos + 1;
            let struct_name = tokenized[parsed_pos];
            let type_def_result = parse_type_definition(&struct_name);

            if type_def_result.is_err() {
                return Err(format!("'struct' definition is malformed: {}: {line}", type_def_result.unwrap_err() ))
            }
            
            let type_def = type_def_result.unwrap();
            parsed_pos += 1;

            let inherits_from = parsed_pos < tokenized_len && tokenized[parsed_pos] == "<:";
            let inherit_details = 
                if inherits_from {
                    parsed_pos += 1;
                    let abs_name = tokenized[parsed_pos];
                    let as_type_result = parse_type_definition(abs_name);
                    if as_type_result.is_err() {
                        return Err(format!("'struct definition is malformed: {}: {line}", as_type_result.unwrap_err()))
                    }

                    let as_type = as_type_result.unwrap();
                    Some(AbstractType { struct_name: as_type.name, generic_arguments: as_type.generic_args })
                } else {
                    None
                };

            Ok(ParsedStruct { 
                is_mutable, 
                struct_name: type_def.name.to_string(), 
                fields: Vec::new(), 
                inherits_from: inherit_details,
                generic_arguments: type_def.generic_args
             })
        },
        Some(_) => {
            Err(format!("'struct' definition does not contain a name: {line}"))
        },
        None => Err(format!("'struct' identifier not found as part of the line: {line}"))
    }

}

pub fn parse_struct(lines: &Vec<&str>, line_number: &mut u32) -> Result<ParsedStruct, String> {
    fn parse_struct_fields(struct_def: &mut ParsedStruct, lines: &Vec<&str>, line_number: &mut u32) {
        let to_skip: usize = (*line_number).try_into().unwrap();
        
        for line in lines.iter().skip(to_skip) {
            *line_number += 1;

            let trimmed = line.trim();
            if trimmed == "end" {
                return;
            }

            let field_result = parse_field(line, *line_number);
            if field_result.is_err() {
                println!("Error Parsing Struct Fields::{}", field_result.clone().unwrap_err());
            }

            let field_def = field_result.unwrap();
            struct_def.fields.push(field_def);
        } 
    }

    let mut structs = Vec::new();

    for line in lines.iter() {
        let struct_res = parse_struct_part(line);
        *line_number += 1;

        match struct_res {
            Ok(mut parsed_struct) => {
                parse_struct_fields(&mut parsed_struct, &lines, line_number);
                structs.push(parsed_struct);
            },
            Err(_) => {  }
        };
    };
    
    return Ok(structs[0].clone());
}

pub fn are_structs_equal(lhs: &ParsedStruct, rhs: &ParsedStruct) -> bool {
    if lhs.is_mutable != rhs.is_mutable { return false; }
    if lhs.struct_name != rhs.struct_name { return false; }
    if lhs.fields != rhs.fields { return false; }
    if lhs.generic_arguments != rhs.generic_arguments { return false; }
    if lhs.inherits_from != rhs.inherits_from { return false; }
    return true;
}