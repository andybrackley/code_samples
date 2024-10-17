#[cfg(test)]
mod tests {
    use generator::parser::parser_structs::{parse_struct, parse_struct_name, parse_struct_part};

    #[test] 
    pub fn parse_of_simple_struct_def_test() {
        let line = "Base.@kwdef mutable struct SomeGeneric <: AbstractT{Int64}";
        
        let result = parse_struct_part(line);
        println!("parsed struct def: {:#?}", result);

        assert!(result.is_ok());
    }

    #[test] 
    pub fn parse_generic_struct_def_test() {
        // let line = "SomeGeneric{T, U}";
        let line = "NonGeneric";

        let result = parse_struct_name(line);
        println!("parsed struct name: {:#?}", result);

        assert!(result.is_ok());

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
    
        assert!(result.is_ok());
    }
}