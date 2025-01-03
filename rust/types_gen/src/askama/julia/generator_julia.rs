// https://rinja-rs.github.io/askama/template_syntax.html

use std::fs::File;
use std::io::Write;

use crate::{
    askama::common::{ Field, StructDefDetails },
    common::parser_types::{ ParsedType, ParsedVariableType },
    nom::parser::Parser,
};

use askama::Template;

#[derive(Template)]
#[template(path = "julia_template_struct.txt", block = "struct_def")]
struct StructJuliaDefTemplate<'a> {
    pub struct_def: &'a StructDefDetails,
}

pub struct GeneratorJulia {}
impl GeneratorJulia {
    fn format_var_type(typ: &ParsedVariableType) -> String {
        let gen = typ.generic_args
            .iter()
            .map(|a| Self::format_var_type(a))
            .collect::<Vec<String>>()
            .join(", ");

        if gen.is_empty() {
            typ.name.to_string()
        } else {
            format!("{}{{{}}}", typ.name, gen)
        }
    }

    fn generate_type(parsed_type: &ParsedType) -> String {
        match parsed_type {
            ParsedType::Struct(struct_def) => {
                let detail = StructDefDetails::from_parsed(struct_def, &Self::format_var_type);
                let template = StructJuliaDefTemplate {
                    struct_def: &detail,
                };

                template.render().unwrap()
            }
            _ => "".to_string(),
        }
    }

    pub fn generate_file(base_path: &str, file_name: &str, parsed: &Parser) -> Result<(), String> {
        let mut lines: Vec<String> = Vec::new();

        parsed
            .get_types()
            .iter()
            .for_each(|t| {
                let r = Self::generate_type(t);
                lines.push(r);
                lines.push("".to_string());
            });

        if lines.is_empty() {
            return Err("Nothing to write".to_string());
        }

        let output_file_path = format!("{}julia/{}.jl", base_path, file_name);

        let mut file = File::create(&output_file_path).map_err(|e|
            format!("Failed to create file: {}-{:?}", &output_file_path, e)
        )?;

        for line in lines {
            writeln!(file, "{}", line).map_err(|e|
                format!("Failed to write line to file: {}-{:?}", &output_file_path, e)
            )?;
        }
        println!("Completed writing file: {}", &output_file_path);
        Ok(())
    }
}

fn format_as(value: &Field, format: &str) -> String {
    format!("{}::{}", value.field, value.typ)
}
