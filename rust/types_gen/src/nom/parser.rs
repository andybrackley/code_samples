use std::rc::Rc;

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
    pub file_name: Option<String>,
    all_types: Vec<Rc<ParsedType>>,
}

impl Parser {
    pub fn from_lines(lines: String) -> Result<Parser, String> {
        match parse_all_types(&lines) {
            Ok((_, types)) => Ok(Parser {
                file_name: None,
                all_types: types.iter().map(|t| Rc::new(t.clone())).collect(),
            }),
            Err(err) => Err(format!("Error parsing types: {:?}", err)),
        }
    }

    pub fn from_file(base: &str, filename: &str, ext: &str) -> Result<Parser, String> {
        let full_path = format!("{}{}.{}", base, filename, ext);

        let contents = read_file(&full_path)
            .map_err(|open_err| format!("Error opening file: {:?}", open_err))?;

        Parser::from_lines(contents).map(|p| Parser {
            file_name: Some(filename.to_string()),
            all_types: p.all_types,
        })
    }

    pub fn get_types(&self) -> &Vec<Rc<ParsedType>> {
        &self.all_types
    }
}
