#[cfg(test)]
pub mod test_generate_julia {
    use std::env;

    use types_gen::{
        askama::julia::generator_julia::GeneratorJulia,
        nom::{parser::Parser, parser_env::ParserEnv},
    };

    fn generate_files(name: Vec<&str>) {
        let base_path = "./idl/";
        let base_output_path = "./generated/";

        match env::current_dir() {
            Ok(path) => println!("The current directory is: {}", path.display()),
            Err(e) => println!("Error getting current directory: {}", e),
        }

        let parsed = name
            .into_iter()
            .map(|n| Parser::from_file(base_path, n, "jl").unwrap())
            .collect::<Vec<Parser>>();

        let parser_env = ParserEnv::build_from(&parsed.iter().collect());
        dbg!(&parser_env.var_sized_types);

        GeneratorJulia::generate_files(base_output_path, &parser_env).unwrap();
    }

    #[test]
    pub fn run_test_file() {
        generate_files(vec!["tests"]);
    }

    #[test]
    pub fn generate() {
        generate_files(vec!["common", "graph_message"]);
    }
}
