#[cfg(test)]
mod tests {
    use generator::parser::{parsed_types::ParsedVariableType, parser_structs::{parse_struct, parse_struct_part}};

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

        let lines: Vec<&str> = lines_str.split_ascii_whitespace().collect();
        let result = parse_struct(&lines, &mut 0);
    
        assert!(result.is_ok(), "Failed to parse struct with error: {}", result.unwrap_err());

        println!("ParsedStruct: {:#?}", result.unwrap());
    }
}