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

use std::collections::HashSet;
use std::fs;
// Slow serialization of some types
// Writing functions for serialization
//    Equality Operators
//    Analytic Node
//    Dictionary<String, CustomStructure> // 700 long...
use std::io::Write;
use std::{fs::File, rc::Rc};

use crate::askama::common::EnumDefDetails;
use crate::common::parser_types::{EnumType, ParsedField};
use crate::{
    askama::common::StructDefDetails,
    common::parser_types::{ParsedStruct, ParsedType, ParsedVariableType},
    nom::parser_env::ParserEnv,
};

use askama::Template;

mod julia_formatters {
    use std::rc::Rc;

    use crate::{
        askama::common::StructDefDetails,
        common::{
            field_positions::FieldPositions,
            parser_types::{ParsedField, ParsedVariableType},
        },
    };

    pub fn format_generics_impl(
        args: &Vec<Box<ParsedVariableType>>,
        mapper: &dyn Fn(&str) -> String,
    ) -> String {
        let gen_str = args
            .iter()
            .map(|a| format_var_type(a, mapper))
            .collect::<Vec<String>>()
            .join(", ");

        if gen_str.is_empty() {
            gen_str
        } else {
            format!("{{{}}}", gen_str)
        }
    }

    pub fn format_var_type(typ: &ParsedVariableType, mapper: &dyn Fn(&str) -> String) -> String {
        let name = mapper(&typ.name);
        format!(
            "{}{}",
            name,
            format_generics_impl(&typ.generic_args, mapper)
        )
    }

    pub fn as_index_field_name(struct_def: &StructDefDetails, field: &Rc<ParsedField>) -> String {
        format!("{}_INDEX", field.field_name.to_uppercase())
    }

    pub fn calc_index_pos(field_positions: &FieldPositions, field: &Rc<ParsedField>) -> String {
        let pos = field_positions
            .offset_fields()
            .iter()
            .position(|f| f.field_name == field.field_name)
            .unwrap();

        format!("{}", pos + 2) // 1 based index + 1 for the object size storage
    }
}

#[derive(Template)]
#[template(path = "./julia/julia_template_struct.txt", block = "struct_def")]
struct StructJuliaDefTemplate<'a> {
    pub env: &'a ParserEnv,
    parsed_struct: &'a ParsedStruct,
    pub struct_def: &'a StructDefDetails,
}
impl<'a> StructJuliaDefTemplate<'a> {
    pub fn dependent_types(&self) -> HashSet<String> {
        self.parsed_struct
            .fields
            .iter()
            .flat_map(|f| f.field_type.flatten())
            .filter(|s| self.env.type_lookup.contains_key(s))
            .collect::<HashSet<String>>()
    }

    pub fn format_generics(args: &Vec<Box<ParsedVariableType>>) -> String {
        julia_formatters::format_generics_impl(args, &|s| s.to_string())
    }

    pub fn format_var_type(typ: &ParsedVariableType) -> String {
        julia_formatters::format_var_type(typ, &|s| s.to_string())
    }

    pub fn format_reader_var_type(&self, typ: &ParsedVariableType) -> String {
        self.format_buffer_var_type(typ, "Reader")
    }

    pub fn format_writer_var_type(&self, typ: &ParsedVariableType) -> String {
        // self.format_buffer_var_type(typ, "Writer")

        let types = julia_formatters::format_var_type(typ, &|s| match s {
            "Array" => "BufferedArray.Writer".to_string(),
            "Vector" => "BufferedArray.Writer".to_string(),
            s => {
                if self.env.type_lookup.contains_key(s) {
                    format!("BufferedObj.Writer{{{}}}", s)
                } else {
                    s.to_string()
                }
            }
        });

        if types.starts_with("BufferedArray") {
            types
        } else if types.starts_with("Union") || types.starts_with("Optional") {
            format!("BufferedObj.UnionWriter{{{}}}", types)
        } else if types.starts_with("BufferedObj.Writer") {
            types
        } else {
            format!("BufferedObj.Writer{{{}}}", types)
        }
    }

    fn format_buffer_var_type(&self, typ: &ParsedVariableType, x: &str) -> String {
        let types = julia_formatters::format_var_type(typ, &|s| match s {
            "Array" => "BufferedArray.Instance".to_string(),
            "Vector" => "BufferedArray.Instance".to_string(),
            s => {
                if self.env.type_lookup.contains_key(s) {
                    format!("{}_{}", s, x)
                } else {
                    s.to_string()
                }
            }
        });

        types
    }

    pub fn as_offset_field_name(&self, field: &Rc<ParsedField>) -> String {
        format!("{}_OFFSET", field.field_name.to_uppercase())
    }

    pub fn as_index_field_name(&self, field: &Rc<ParsedField>) -> String {
        julia_formatters::as_index_field_name(&self.struct_def, field)
    }

    pub fn calc_offset(&self, field: &Rc<ParsedField>) -> String {
        let fields = self.struct_def.field_positions.non_offset_fields();

        let pos = fields
            .iter()
            .position(|f| f.field_name == field.field_name)
            .unwrap();

        let current = fields[pos].clone();
        let prev = if pos == 0 { None } else { fields.get(pos - 1) };

        match prev {
            Some(p) => {
                format!(
                    "{} + serialized_size({})",
                    self.as_offset_field_name(p),
                    Self::format_var_type(&p.field_type)
                )
            }
            None => {
                format!("START_OFFSET")
            }
        }
    }

    pub fn get_end_offset(&self) -> String {
        let binding = &self.struct_def.field_positions.fixed_size_fields;
        let field = binding.last();
        match field {
            None => "START_OFFSET".to_string(),
            Some(f) => {
                format!(
                    "{} + serialized_size({})",
                    self.as_offset_field_name(f),
                    Self::format_var_type(&f.field_type)
                )
            }
        }
    }

    pub fn calc_index_pos(&self, field: &Rc<ParsedField>) -> String {
        julia_formatters::calc_index_pos(&self.struct_def.field_positions, field)
    }
}

#[derive(Template)]
#[template(path = "./julia/julia_templates.txt", block = "enum_def")]
struct EnumJuliaDefTemplate<'a> {
    pub enum_def: &'a EnumDefDetails,
}

pub struct GeneratorJulia {}
impl GeneratorJulia {
    fn generate_enum(parsed_enum: &EnumType) -> String {
        let detail = EnumDefDetails::from_parsed(parsed_enum);

        let enum_def = (EnumJuliaDefTemplate { enum_def: &detail })
            .render()
            .unwrap();

        enum_def
    }

    fn generate_struct(parsed_struct: &ParsedStruct, env: &ParserEnv) -> String {
        let detail = StructDefDetails::from_parsed(parsed_struct, &env.var_sized_types);

        let struct_def = (StructJuliaDefTemplate {
            env,
            parsed_struct,
            struct_def: &detail,
        })
        .render()
        .unwrap();
        // let struct_buffer_def = (StructBufferJuliaDefTemplate {
        //     env: &env,
        //     struct_def: &detail,
        // })
        // .render()
        // .unwrap();

        let mut lines: Vec<String> = Vec::new();
        lines.push(struct_def);
        lines.push("".to_string());
        // lines.push(struct_buffer_def);

        lines.join("\r\n")
    }

    fn generate_type(parsed_type: &ParsedType, env: &ParserEnv) -> String {
        match parsed_type {
            ParsedType::Struct(struct_def) => Self::generate_struct(struct_def, env),
            ParsedType::Enum(enum_def) => Self::generate_enum(enum_def),

            _ => "".to_string(),
        }
    }

    pub fn generate_files(base_path: &str, parsed: &ParserEnv) -> Result<(), String> {
        let mut includes = Vec::new();
        // includes.push("using Base: memcmp".to_string());
        includes.push("using ..Framework".to_string());
        includes.push("".to_string());

        let base_path = format!("{}/julia/", base_path);

        parsed.all_types.iter().for_each(|file| {
            let file_name = file.filename.as_ref().unwrap();
            let types = &file.types;
            includes.push(format!("include(\"{}/{}.jl\")", file_name, file_name));

            Self::generate_file_impl(&base_path, file_name, parsed, types).unwrap();
        });

        let output_file_path = format!("{}/lib.jl", base_path);
        let mut file = File::create(&output_file_path)
            .map_err(|e| format!("Failed to create file: {}-{:?}", &output_file_path, e))?;

        for include in includes {
            writeln!(file, "{}", include).map_err(|e| {
                format!(
                    "Failed to write line to file: {}-{:?}",
                    &output_file_path, e
                )
            })?;
        }

        println!("Completed writing file: {}", &output_file_path);
        Ok(())
    }

    fn write_lines(
        lines: &Vec<String>,
        parsed_type: &ParsedType,
        julia_base_path: &str,
        parsed_file_name: &str,
    ) -> Result<(), String> {
        let typename = parsed_type.get_type_name();

        if lines.is_empty() {
            let message = format!(
                "No lines genereated for type: '{}' defined in file: '{}'",
                typename, parsed_file_name
            );

            return Err(message);
        }

        let output_file_path = format!("{}/{}.jl", julia_base_path, typename);

        let mut file = File::create(&output_file_path)
            .map_err(|e| format!("Failed to create file: {}-{:?}", &output_file_path, e))?;

        let results: Result<(), String> = lines.iter().try_for_each(|line| {
            writeln!(file, "{}", line).map_err(|e| {
                let message = format!(
                    "Failed to write line to file: {}-{:?}",
                    &output_file_path, e
                );
                message
            })
        });

        if results.is_ok() {
            println!("Completed writing file: '{}'", &output_file_path);
        }
        return results;
    }

    fn write_lib_file(
        base_path: &str,
        file_name: &str,
        to_include: &Vec<String>,
    ) -> Result<(), String> {
        let output_file_path = format!("{}{}/{}.jl", base_path, file_name, file_name);

        let mut file = File::create(&output_file_path)
            .map_err(|e| format!("Failed to create file: {}-{:?}", &output_file_path, e))?;

        let results = to_include.iter().try_for_each(|include| {
            writeln!(file, "{}", format!("include(\"{}.jl\")", include)).map_err(|e| {
                format!(
                    "Failed to write line to file: {}-{:?}",
                    &output_file_path, e
                )
            })
        });

        if results.is_ok() {
            println!("Completed code generation for: '{}'", &file_name);
        }

        return results;
    }

    fn generate_file_impl(
        base_path: &str,
        file_name: &str,
        env: &ParserEnv,
        types: &Vec<Rc<ParsedType>>,
    ) -> Result<(), String> {
        let mut to_include: Vec<String> = Vec::new();

        let output_dir = format!("{}{}", base_path, file_name);
        fs::create_dir_all(output_dir.clone())
            .map_err(|e| format!("Failed to create directory: '{}', Err: {}", output_dir, e))?;

        let results = types.iter().try_for_each(|t| {
            let mut lines: Vec<String> = Vec::new();

            let r = Self::generate_type(t, env);
            lines.push(r);
            lines.push("".to_string());
            Self::write_lines(&lines, &t, &output_dir, file_name)?;

            to_include.push(t.get_type_name().to_string());
            Ok(())
        });

        results.and_then(|res| Self::write_lib_file(base_path, file_name, &to_include))
    }
}
