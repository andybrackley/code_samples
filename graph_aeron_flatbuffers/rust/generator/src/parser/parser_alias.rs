use super::{parsed_types::AliasType, parser_types::parse_type_definition, string_utils::split_preserving_braces};

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
