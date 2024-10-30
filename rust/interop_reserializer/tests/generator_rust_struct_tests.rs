#[cfg(test)]
pub mod generator_rust_struct_tests {
    use interop_reserializer::{
        generator_rust::generate_rust_struct::generate_rust_struct,
        lexer::Lexer,
        parser::parser_struct::parse_struct_def,
    };

    fn compare(expected: &str, actual: &str) {
        assert!(expected.trim() == actual.trim(), "e: {}, a: {}", expected, actual);
    }

    fn run_test(julia_def: &str, rust_str: &str) {
        let lexer = Lexer::parse(julia_def);
        let parsed = parse_struct_def(&lexer.tokens, &mut 0);
        let generated = generate_rust_struct(&parsed.unwrap());
        compare(&rust_str, &generated.unwrap().join("\n"));
    }

    #[test]
    pub fn test_struct_with_mapped_types() {
        let julia =
            r#"
mutable struct Level
    asInt32:: Int32
    asInt64:: Int64 
    asFloat32:: Float32
    asFloat64:: Float64
    asString:: String
end"#;

        let rust =
            r#"
pub struct Level {
    pub as_int32: i32,
    pub as_int64: i64,
    pub as_float32: f32,
    pub as_float64: f64,
    pub as_string: String,
}"#;
        run_test(julia, rust);
    }

    #[test]
    pub fn test_struct_with_gen_args() {
        let julia =
            r#"
mutable struct BookUpdate{A, B, C}
    bids::Vector{A}
    mids::Vector{B}
    asks::Vector{C}
end"#;

        let rust =
            r#"
pub struct BookUpdate<A, B, C> {
    pub bids: Vec<A>,
    pub mids: Vec<B>,
    pub asks: Vec<C>,
}"#;
        run_test(julia, rust);
    }

    #[test]
    pub fn test_struct_gen_args_get_mapped() {
        let julia =
            r#"
mutable struct BookUpdate{A, B}
    bids::Vector{A}
    asks::Vector{B}
    mids::Vector{Vector{Float64}}
end"#;

        let rust =
            r#"
pub struct BookUpdate<A, B> {
    pub bids: Vec<A>,
    pub asks: Vec<B>,
    pub mids: Vec<Vec<f64>>,
}"#;
        run_test(julia, rust);
    }
}
