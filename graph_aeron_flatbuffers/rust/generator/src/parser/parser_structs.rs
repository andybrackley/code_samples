use crate::parser::parser_types::parse_field;

use super::{parsed_scope::ParsedErrorTypes, parsed_types::{AbstractType, ParsedStruct}, parser_types::parse_type_definition, string_utils::split_preserving_braces};

// pub fn parse_struct_part(line: &str) -> Result<ParsedStruct, ParsedErrorTypes> {}
// pub fn parse_struct_line(line: &str) -> Result<Scope, ParsedErrorTypes> {} 


/// struct Person
/// mutable struct Person
/// mutable struct Person <: AbstractPerson
pub fn parse_struct_part(line: &str) -> Result<ParsedStruct, ParsedErrorTypes> {
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
                return Err(ParsedErrorTypes::Malformed(format!("'struct' definition is malformed: {}: {line}", type_def_result.unwrap_err() )))
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
                        return Err(ParsedErrorTypes::Malformed(format!("'struct definition is malformed: {}: {line}", as_type_result.unwrap_err())))
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
            Err(ParsedErrorTypes::Malformed(format!("'struct' definition does not contain a name: {line}")))
        },
        None => Err(ParsedErrorTypes::Malformed(format!("'struct' identifier not found as part of the line: {line}")))
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

// pub fn are_structs_equal(lhs: &ParsedStruct, rhs: &ParsedStruct) -> bool {
//     if lhs.is_mutable != rhs.is_mutable { return false; }
//     if lhs.struct_name != rhs.struct_name { return false; }
//     if lhs.fields != rhs.fields { return false; }
//     if lhs.generic_arguments != rhs.generic_arguments { return false; }
//     if lhs.inherits_from != rhs.inherits_from { return false; }
//     return true;
// }


pub mod scope_struct {
    use crate::parser::parsed_scope::Scope;

    pub fn try_create_struct_scope(line: &str, parent_scope: Scope) -> Option<&str> {
        return None
    }

    pub fn process_line(line: &str, /* need the parent Scope::Struct */) {

    }


}