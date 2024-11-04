#[cfg(test)]
pub mod full_julia_to_rust_tests {
    use std::{ fs::File, io::{ self, BufReader, Read, Write } };

    use interop_reserializer::{
        generator_rust::generate_rust_struct::generate_rust_struct,
        lexer::Lexer,
        parser::parser::Parser,
        parser_types::ParsedType,
    };

    const INPUT_DIR: &str = ".\\julia_samples\\";
    const OUTPUT_DIR: &str = ".\\generated\\src\\";

    fn run(julia_file_name: &str) -> io::Result<()> {
        let julia_file_path = format!("{}{}", INPUT_DIR, julia_file_name);

        let file = File::open(&julia_file_path)?;
        let mut reader = BufReader::new(file);

        let mut lines = String::new();
        reader.read_to_string(&mut lines)?;

        let lexer = Lexer::parse(&lines);
        let parsed = Parser::parse(&lexer.tokens);
        let un = parsed.unwrap();

        let mut to_output = Vec::new();
        for s in un.tokens {
            match s {
                ParsedType::Struct(s) => {
                    let r = generate_rust_struct(&s);
                    match r {
                        Ok(l) => {
                            to_output.extend(l);
                        }
                        Err(e) => {
                            print!(
                                "Failed to generate struct: '{}' with Err: '{}'",
                                s.struct_name,
                                e
                            );
                            assert!(false, "{}", e);
                        }
                    }
                }
                _ => {}
            }
        }

        if !to_output.is_empty() {
            let rust_output_file = julia_file_name.replace(".jl", ".rs");
            let rust_output_file_path = format!("{}{}", OUTPUT_DIR, rust_output_file);

            let mut file = File::create(rust_output_file_path)?;
            for line in to_output {
                writeln!(file, "{}", line)?;
            }
        }

        return Ok(());
    }

    #[test]
    pub fn test_types_test_jl() {
        let file_name = "types_tests.jl";
        run(&file_name).unwrap();
    }
}
