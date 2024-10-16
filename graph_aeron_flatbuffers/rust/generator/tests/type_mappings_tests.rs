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


#[cfg(test)]
mod tests {
    use generator::*;
    use parser::{parsed_types::{ParsedField, ParsedVariableType}, types_outputter::output};

    #[test]
    fn invalid_mapping_gives_error() {
        let not_enough_args = parser::types_parser::parse_field("field_name", 0);
        assert!(not_enough_args.is_err());

        let too_many_args = parser::types_parser::parse_field("field_name::type::extra", 0);
        assert!(too_many_args.is_err());
    }

    fn test_valid_type_mapping(line: &str, expected: &ParsedField) {
        let t1 = parser::types_parser::parse_field(line, 0);
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

    #[test]
    pub fn test_output_of_complex_type() {
        let line = "multi_generics::Union{Vector{Union{Int64, TimeStamp, String}}, BookUpdateType}";
        println!("Julia: {}", line);

        let parsed = parser::types_parser::parse_field(line, 0);
        output(parsed.unwrap());
    }
}
