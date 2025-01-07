// DONE:
// - [ x ] Find Julia Frameworks for serialization
//
// TODO:
// - [ ] Add support for enums
// - [ ] Serialize
// - [ ] Deserialize
// - [ ] Setters
// - [ ] Buffered Struct Version
// - [ ] Var sized fields ( Offsets )
// - [ ] Clone method
// - [ ] Inherit
// - [ ] Aliases / Const

// https://rinja-rs.github.io/askama/template_syntax.html

use std::fs::File;
use std::io::Write;

use crate::{
    askama::common::StructDefDetails,
    common::parser_types::{ ParsedStruct, ParsedType, ParsedVariableType },
    nom::parser::Parser,
};

use askama::Template;

mod julia_formatters {
    use crate::common::parser_types::ParsedVariableType;

    pub fn format_generics_impl(args: &Vec<Box<ParsedVariableType>>) -> String {
        let gen_str = args
            .iter()
            .map(|a| format_var_type(a))
            .collect::<Vec<String>>()
            .join(", ");

        if gen_str.is_empty() {
            gen_str
        } else {
            format!("{{{}}}", gen_str)
        }
    }

    pub fn format_var_type(typ: &ParsedVariableType) -> String {
        format!("{}{}", typ.name, format_generics_impl(&typ.generic_args))
    }
}

#[derive(Template)]
#[template(path = "julia_template_struct.txt", block = "struct_def")]
struct StructJuliaDefTemplate<'a> {
    pub struct_def: &'a StructDefDetails,
}
impl<'a> StructJuliaDefTemplate<'a> {
    pub fn format_generics(args: &Vec<Box<ParsedVariableType>>) -> String {
        julia_formatters::format_generics_impl(args)
    }

    pub fn format_var_type(typ: &ParsedVariableType) -> String {
        julia_formatters::format_var_type(typ)
    }
}

#[derive(Template)]
#[template(path = "julia_template_struct.txt", block = "struct_buffer_def")]
struct StructBufferJuliaDefTemplate<'a> {
    pub struct_def: &'a StructDefDetails,
}
impl<'a> StructBufferJuliaDefTemplate<'a> {
    pub fn format_generics(args: &Vec<Box<ParsedVariableType>>) -> String {
        julia_formatters::format_generics_impl(args)
    }

    pub fn format_var_type(typ: &ParsedVariableType) -> String {
        julia_formatters::format_var_type(typ)
    }
}

pub struct GeneratorJulia {}
impl GeneratorJulia {
    fn generate_struct(parsed_struct: &ParsedStruct) -> String {
        let detail = StructDefDetails::from_parsed(parsed_struct);

        let struct_def = (StructJuliaDefTemplate { struct_def: &detail }).render().unwrap();
        let struct_buffer_def = (StructBufferJuliaDefTemplate { struct_def: &detail })
            .render()
            .unwrap();

        let mut lines: Vec<String> = Vec::new();
        lines.push(struct_def);
        lines.push("".to_string());
        lines.push(struct_buffer_def);

        lines.join("\r\n")
    }

    fn generate_type(parsed_type: &ParsedType) -> String {
        match parsed_type {
            ParsedType::Struct(struct_def) => { Self::generate_struct(struct_def) }
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
