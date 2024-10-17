use super::{parsed_types::{AbstractType, ParsedStruct, ParsedVariableType}, parser_types::{parse_field, parse_type_definition}};

pub fn parse_struct_name(name: &str) -> Result<String, String> {
    let as_types = parse_type_definition(name);

    match as_types {
        Ok(t) => {
            Ok(t.name)
        },
        Err(err) => Err(err)
    }
}

/// struct Person
/// mutable struct Person
/// mutable struct Person <: AbstractPerson
pub fn parse_struct_part(line: &str) -> Result<ParsedStruct, String> {
    let tokenized: Vec<&str> = line.split_whitespace().collect();
    let struct_id_pos = tokenized.iter().position(|&token| token == "struct");
    let tokenized_len = tokenized.len();
    
    match struct_id_pos {
        Some(pos) if pos < tokenized_len => {
            let is_mutable = pos != 0 && tokenized[pos - 1] == "mutable";
            
            let mut parsed_pos = pos + 1;
            let struct_name = tokenized[parsed_pos];
            let type_def = parse_type_definition(struct_name);

            println!("TypeDef::{:#?}", type_def);

//             let as_field = parse_field(tokenized[parsed_pos], 0);
            parsed_pos += 1;

            let inherits_from = parsed_pos < tokenized_len && tokenized[parsed_pos] == "<:";
            let inherit_details = 
                if inherits_from {
                    Some(AbstractType { struct_name: "Test".to_string(), generic_arguments: Vec::new() })
                } else {
                    None
                };

            Ok(ParsedStruct { 
                is_mutable, 
                struct_name: struct_name.to_string(), 
                fields: Vec::new(), 
                inherits_from: inherit_details,
                generic_arguments: Vec::new()
             })
        },
        Some(_) => {
            Err(format!("'struct' definition does not contain a name: {line}"))
        },
        None => Err(format!("'struct' identifier not found as part of the line: {line}"))
    }

}

pub fn parse_struct(lines: &Vec<&str>, line_number: &mut u32) -> Result<ParsedStruct, String> {

    for line in lines.iter() {

        *line_number += 1;
    }



    
    return Err("Not Implemented".to_string());
}