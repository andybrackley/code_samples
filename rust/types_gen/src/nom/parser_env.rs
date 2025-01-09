use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::common::parser_types::ParsedType;

use super::parser::Parser;

pub struct FileDefinitions {
    pub filename: Option<String>,
    pub types: Vec<Rc<ParsedType>>,
}

pub struct ParserEnv {
    pub all_types: Vec<FileDefinitions>,
    pub type_lookup: HashMap<String, Rc<ParsedType>>,
    pub var_sized_types: HashSet<String>,
}
impl ParserEnv {
    pub fn var_sized_types() -> HashSet<String> {
        HashSet::from([
            "String".to_string(),
            "Vector".to_string(),
            "Array".to_string(),
        ])
    }

    fn is_type_var_sized(lookup: &HashSet<String>, typ: &ParsedType) -> bool {
        let type_names: HashSet<String> = typ.get_all_types();
        let exists = type_names.iter().any(|typ| lookup.contains(typ));
        exists
    }

    pub fn build_from(parsers: &Vec<&Parser>) -> Self {
        let mut all_types = Vec::new();
        let mut type_lookup = HashMap::new();
        let mut var_sized_types = Self::var_sized_types();

        for parser in parsers {
            let mut parser_types = Vec::new();

            for parsed_type in parser.get_types() {
                let rc = parsed_type.clone();

                parser_types.push(rc.clone());

                let type_name = rc.get_type_name();
                if type_lookup.contains_key(&type_name) {
                    panic!("Duplicate type name: {}", type_name);
                }

                type_lookup.insert(type_name.to_string(), rc.clone());

                if Self::is_type_var_sized(&var_sized_types, parsed_type) {
                    var_sized_types.insert(type_name.to_string());
                }
            }

            all_types.push(FileDefinitions {
                filename: parser.file_name.clone(),
                types: parser_types,
            });
        }
        ParserEnv {
            all_types,
            type_lookup,
            var_sized_types,
        }
    }
}
