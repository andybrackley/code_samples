use crate::common::parser_types::ParsedType;

use super::parser_impl::parse_all_types;

fn read_file(full_path: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(full_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct Parser {
    all_types: Vec<ParsedType>,
}

impl Parser {
    pub fn from_lines(lines: String) -> Result<Parser, String> {
        match parse_all_types(&lines) {
            Ok((_, types)) => Ok(Parser { all_types: types }),
            Err(err) => Err(format!("Error parsing types: {:?}", err)),
        }
    }

    pub fn from_file(base: &str, filename: &str) -> Result<Parser, String> {
        let full_path = format!("{}{}", base, filename);

        let contents = read_file(&full_path).map_err(|open_err|
            format!("Error opening file: {:?}", open_err)
        )?;

        Parser::from_lines(contents)
    }

    pub fn get_types(&self) -> &Vec<ParsedType> {
        &self.all_types
    }
}
