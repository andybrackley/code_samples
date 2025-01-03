#[cfg(test)]
pub mod test_generate_julia {
    use types_gen::nom::parser::Parser;

    #[test]
    pub fn generate() {
        let base_path = "./idl/";

        let common = Parser::from_file(base_path, "common.jl").unwrap();
        dbg!(common.get_types());

        let graph_message = Parser::from_file(base_path, "graph_message.jl").unwrap();
        dbg!(graph_message.get_types());
    }
}
