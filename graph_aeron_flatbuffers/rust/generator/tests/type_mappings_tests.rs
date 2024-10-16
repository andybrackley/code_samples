// mod type_mappings;
// use type_mappings::*;


/* To Parse

    namespace XXXX

   enum Test 
      v1 = 0,
      v2 = 0
   end

    const Alias = Int64

    struct ImmutableS 

    end

    mutable struct MutableS 

    end


    i64::Int64;

    const Optional = Union{T, Nothing} 
    Vector{T}

*/

#[derive(Debug, Clone)]
enum ParsedVariableType {
    Scaler(String),
    Generic(String, Vec<Box<ParsedVariableType>>)
}

impl ParsedVariableType { 
    fn compare(&self, rhs: &ParsedVariableType) -> bool {
        match (self, rhs) {
            (ParsedVariableType::Scaler(n), ParsedVariableType::Scaler(r_name)) => n == r_name,
            (ParsedVariableType::Generic(n, typ), ParsedVariableType::Generic(r_name, r_type )) => {
                if n != r_name { return false; }
                if typ.len() != r_type.len() { return false; }

                for(t, rt) in typ.iter().zip(r_type.iter()) {
                    if !t.compare(rt) {
                        return false;
                    }
                }

                return true
            }
            _ => false
        } 
    }
}



#[derive(Debug, Clone)]
struct ParsedField {
    field_name: String,
    field_type: ParsedVariableType
}

mod test_code {
    use std::collections::VecDeque;
    use crate::ParsedVariableType;
}


mod generator_impl {
    use crate::{ParsedField, ParsedVariableType};

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

            let v = 
                if types.is_empty() {
                    ParsedVariableType::Scaler(name.to_string())
                } else {
                    ParsedVariableType::Generic(name, types)
                };

            return v;
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
        // scope.types.push(Box::new(v));
        return v;
    }
                    
    pub fn parse_type_definition(field_type_str: &str) -> Result<ParsedVariableType, String> {
        let field_type = parse_generic_type(field_type_str);
        return Ok(field_type);
    }

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

}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::{generator_impl, ParsedField, ParsedVariableType};

    #[test]
    fn invalid_mapping_gives_error() {
        let not_enough_args = generator_impl::parse_field("field_name", 0);
        assert!(not_enough_args.is_err());

        let too_many_args = generator_impl::parse_field("field_name::type::extra", 0);
        assert!(too_many_args.is_err());
    }

    fn test_valid_type_mapping(line: &str, expected: &ParsedField) {
        let t1 = generator_impl::parse_field(line, 0);
        assert!(t1.is_ok());

        let field = t1.unwrap();
        assert!(field.field_name == expected.field_name);

        let types_equal = field.field_type.compare(&expected.field_type);
        assert!(types_equal, "Expected type: {:?} does not match actual type: {:?}", expected.field_type, field.field_type)
    }

    #[test]
    fn test_type_scalar_mappings() {
        let test1 = "    test_i64::Int64";

        let expected = ParsedField {
            field_name: "test_i64".to_string(),
            field_type: ParsedVariableType::Scaler("Int64".to_string())
        };

        test_valid_type_mapping(test1, &expected);        

        let array = "    bids::Vector{Level}";
    }

    #[test]
    fn test_type_optional_mapping() {
        let optional = "    timestamp_exch::Optional{Timestamp}";
        let optional_expected = ParsedField {
            field_name: "timestamp_exch".to_string(),
            field_type: ParsedVariableType::Generic(
                "Optional".to_string(), 
                vec![ Box::new(ParsedVariableType::Scaler("Timestamp".to_string()))]
            )
        };

        test_valid_type_mapping(&optional, &optional_expected);
    }

    #[test]
    fn test_multiple_generic_cases() {
        let union_line = "test_union::Union{Int64, TimeStamp, String}";
        let union_types = vec![
            Box::new(ParsedVariableType::Scaler("Int64".to_string())),
            Box::new(ParsedVariableType::Scaler("TimeStamp".to_string())),
            Box::new(ParsedVariableType::Scaler("String".to_string())),
        ];

        let union_expected = ParsedField {
            field_name: "test_union".to_string(),
            field_type: ParsedVariableType::Generic("Union".to_string(), union_types)
        };

        test_valid_type_mapping(&union_line, &union_expected);
    }
    
    #[test]
    fn test_multiple_nested_generic_types() {
        let optional = "multi_generics::Union{Vector{Union{Int64, TimeStamp, String}}, BookUpdateType}";
        let inner_union_types = vec![
            Box::new(ParsedVariableType::Scaler("Int64".to_string())),
            Box::new(ParsedVariableType::Scaler("TimeStamp".to_string())),
            Box::new(ParsedVariableType::Scaler("String".to_string())),
        ];

        let inner_union = ParsedVariableType::Generic("Union".to_string(), inner_union_types);

        let vec_types = vec![
            Box::new(ParsedVariableType::Generic("Vector".to_string(), vec!(Box::new(inner_union)))),
            Box::new(ParsedVariableType::Scaler("BookUpdateType".to_string()))
        ];

        let outer_union_types = 
            ParsedVariableType::Generic("Union".to_string(), vec_types);
        
        let optional_expected = ParsedField {
            field_name: "multi_generics".to_string(),
            field_type: outer_union_types
        };

        test_valid_type_mapping(&optional, &optional_expected);
    }
}
