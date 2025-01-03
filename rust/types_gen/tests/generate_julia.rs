#[cfg(test)]
pub mod test_generate_julia {
    use types_gen::{ askama::julia::generator_julia, nom::parser::Parser };

    #[test]
    pub fn run_test_file() {
        let base_path = "./idl/";
        let base_output_path = "./generated/";

        let common = Parser::from_file(base_path, "tests.jl").unwrap();
        generator_julia::GeneratorJulia::generate_file(base_output_path, "test", &common).unwrap();
    }

    #[test]
    pub fn generate() {
        let base_path = "./idl/";
        let base_output_path = "./generated/";

        let common = Parser::from_file(base_path, "common.jl").unwrap();
        generator_julia::GeneratorJulia
            ::generate_file(base_output_path, "common", &common)
            .unwrap();

        let graph_message = Parser::from_file(base_path, "graph_message.jl").unwrap();

        generator_julia::GeneratorJulia
            ::generate_file(base_output_path, "graph_message", &graph_message)
            .unwrap();
    }
}
