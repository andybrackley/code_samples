use std::collections::HashMap;

use crate::{
    parser_types::{ ParsedField, ParsedStruct, ParsedVariableType },
    utils::string_utils::camel_to_snake,
};

type ErrorT = String;
type ReturnT = Result<Vec<String>, ErrorT>;

fn get_standard_mapping(t: &str) -> Result<String, String> {
    let mappings = vec![
        ("Int32", "i32"),
        ("Int64", "i64"),
        ("Float32", "f32"),
        ("Float64", "f64"),
        ("String", "String"),
        ("Optional", "Option"),
        ("Vector", "Vec")
    ];

    // TODO: Shouldn't need to init the map each time.
    let lookkup: HashMap<&str, &str> = mappings.into_iter().collect();
    lookkup
        .get(t)
        .cloned()
        .map(|s| s.to_string())
        .ok_or(format!("Type Mapping not found for: '{t}'"))
}

// struct_args contains the generic types for the struct.
// These will take precedence over our looked up types i.e.
// struct Test<A> { var: A }
//
fn get_mapped_type(t: &str, struct_args: &Vec<String>) -> Result<String, String> {
    let is_arg = struct_args.iter().find(|a| a == &t);
    if is_arg.is_some() {
        return Ok(t.to_string());
    }

    return get_standard_mapping(t);
}

fn get_generic_arg_str(
    arg: &ParsedVariableType,
    struct_args: &Vec<String>
) -> Result<String, String> {
    let mapped = get_mapped_type(&arg.name, struct_args);
    let mut with_gens = Vec::new();

    if !arg.generic_args.is_empty() {
        for gen_arg in &arg.generic_args {
            let as_string = get_generic_arg_str(&gen_arg, struct_args);
            if as_string.is_err() {
                return as_string;
            }
            with_gens.push(as_string.unwrap());
        }
    }

    mapped.map(|m| {
        if with_gens.is_empty() {
            m
        } else {
            let gens_str = with_gens.join(", ");
            format!("{}<{}>", m, gens_str)
        }
    })
}

fn get_field_str(field: &ParsedField, struct_args: &Vec<String>) -> Result<String, String> {
    let field_name = camel_to_snake(&field.field_name);
    let field_type = get_generic_arg_str(&field.field_type, struct_args);

    let full = field_type.map(|ft| format!("pub {}: {}", field_name, ft));
    return full;
}

// This bit get the args for the struct definition i.e.
//    struct Test{A, B, C} -> struct<A, B, C>
fn get_struct_gen_args(gen_args: &Vec<Box<ParsedVariableType>>) -> Vec<String> {
    let args: Vec<String> = if gen_args.is_empty() {
        Vec::new()
    } else {
        let args: Vec<String> = gen_args
            .iter()
            .map(|a| a.name.clone())
            .collect();

        args
    };
    args
}

pub fn generate_rust_struct(parsed: &ParsedStruct) -> ReturnT {
    let spacing = "    ";

    let gen_args = get_struct_gen_args(&parsed.generic_arguments);
    let gen_args_str = if gen_args.is_empty() {
        "".to_string()
    } else {
        let csv = gen_args.join(", ");
        format!("<{}>", csv)
    };

    let struct_def = format!("pub struct {}{} {{", parsed.struct_name, gen_args_str);

    // let field_lines: Vec<String> = parsed.fields
    //     .iter()
    //     .map(|f| format!("{spacing}{}", get_field_str(&f, &gen_args)))
    //     .collect();

    let mut field_lines = Vec::new();
    for field in &parsed.fields {
        let fld = get_field_str(&field, &gen_args);
        match fld {
            Ok(f) => {
                field_lines.push(format!("{}{},", spacing, f));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    let mut lines = Vec::new();
    lines.push(struct_def);
    lines.extend(field_lines);

    lines.push("}".to_string());
    lines.push("\n".to_string());
    return Ok(lines);
}
