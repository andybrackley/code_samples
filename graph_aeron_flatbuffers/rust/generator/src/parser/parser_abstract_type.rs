use super::{parsed_types::AbstractType, parser_types::parse_type_definition, string_utils::split_preserving_braces};

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
