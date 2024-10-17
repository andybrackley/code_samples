#[cfg(test)]
mod tests {
    use generator::parser::{parsed_types::{AbstractType, AliasType, ParsedField, ParsedStruct, ParsedVariableType}, parser_structs::{parse_abstract_type_part, parse_alias_part, parse_struct, parse_struct_part}};

    #[test]
    pub fn parse_alias_type() {
        let line = "const TestAlias{T, U} = TestImpl{T, U}";
        let result = parse_alias_part(line);
        assert!(result.is_ok(), "Parsing Failed with Error: {0}", result.unwrap_err());

        let expected_generics = vec![
            Box::new(ParsedVariableType::scaler("T")),
            Box::new(ParsedVariableType::scaler("U")),
        ];

        let expected = AliasType {
            alias_type: ParsedVariableType::generic("TestAlias", expected_generics.clone()),
            target_type: ParsedVariableType::generic("TestImpl", expected_generics.clone())
        };

        let alias = result.unwrap();
        assert!(expected == alias, "Expected: {:#?} not equal Actual: {:#?}", expected, alias);
    }

    #[test]
    pub fn parse_abstract_type() {
        let line = "abstract type Test{T, U} end";

        let result = parse_abstract_type_part(line);

        assert!(result.is_ok());

        let expected = AbstractType {
            struct_name: "Test".to_string(),
            generic_arguments: vec![
                Box::new(ParsedVariableType::scaler("T")),
                Box::new(ParsedVariableType::scaler("U")),
            ]
        };

        let abs_type = result.unwrap();
        assert!(expected == abs_type, "Expected: {:#?} not equal Actual: {:#?}", expected, abs_type);
    }

    #[test] 
    pub fn parse_of_simple_struct_def_test() {
        let line = "Base.@kwdef mutable struct SomeGeneric{T, U} <: AbstractT{Int64, Float64}";
        
        let result = parse_struct_part(line);
        assert!(result.is_ok());

        let as_struct = result.unwrap();
        let actual_args = 
            ParsedVariableType::generic("", as_struct.generic_arguments);

        let expect_args = 
            ParsedVariableType::generic("", 
            vec![
                Box::new(ParsedVariableType::scaler("T")), 
                Box::new(ParsedVariableType::scaler("U"))
            ]);
        
        assert!(as_struct.struct_name == "SomeGeneric");
        assert!(expect_args.compare(&actual_args));
        assert!(as_struct.is_mutable, "Struct should be mutable");
        
        assert!(as_struct.inherits_from.is_some(), "Struct should be inheriting from AbstractT{{Int64, Float}}");
    
        let expect_inhert = 
            ParsedVariableType::generic("AbstractT", 
                vec![ Box::new(ParsedVariableType::scaler("Int64")),
                Box::new(ParsedVariableType::scaler("Float64") )]);

        let unwrapped = as_struct.inherits_from.unwrap();
        let actual_inherit = 
            ParsedVariableType::generic(&unwrapped.struct_name,unwrapped.generic_arguments);

        assert!(expect_inhert.compare(&actual_inherit), "expected: {:#?}, got: {:#?}", expect_inhert, actual_inherit);
    }

    #[test] 
    pub fn parse_of_simple_struct_test() {
        let lines_str = r#"mutable struct Level
           price::Price
           size::Size
           recent_size::Float64
           last_update::Timestamp
        end
        "#;

        let lines: Vec<&str> = lines_str.split('\n').collect();
        let result = parse_struct(&lines, &mut 0);
    
        assert!(result.is_ok(), "Failed to parse struct with error: {}", result.unwrap_err());
        
        let expected = ParsedStruct {
            is_mutable: true,
            struct_name: "Level".to_string(),
            inherits_from: None, 
            fields: vec![ 
                ParsedField { field_name: "price".to_string(), field_type: ParsedVariableType::scaler("Price") },
                ParsedField { field_name: "size".to_string(), field_type: ParsedVariableType::scaler("Size") },
                ParsedField { field_name: "recent_size".to_string(), field_type: ParsedVariableType::scaler("Float64") },
                ParsedField { field_name: "last_update".to_string(), field_type: ParsedVariableType::scaler("Timestamp") },
            ],
            generic_arguments: Vec::new()
        };
        
        let result_struct = result.unwrap();
        assert!(expected == result_struct, "Structs aren't equal, Expected: {:#?}, actual: {:#?}", expected, result_struct);
    }
}