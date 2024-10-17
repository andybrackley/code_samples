use super::{parsed_types::{AbstractType, ParsedStruct}, parser_types::parse_type_definition, string_utils::split_preserving_braces};

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

pub fn parse_struct(_lines: &Vec<&str>, _line_number: &mut u32) -> Result<ParsedStruct, String> {

    // for line in lines.iter() {

    //     *line_number += 1;
    // }



    
    return Err("Not Implemented".to_string());
}