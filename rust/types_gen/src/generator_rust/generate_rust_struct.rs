use std::collections::HashMap;
use crate::common::parser_types::{AvailTypes, ParsedField, ParsedStruct, ParsedVariableType};
use crate::common::string_utils::camel_to_snake;

type ErrorT = String;
type ReturnT = Result<Vec<String>, ErrorT>;

fn get_standard_mapping(t: &str) -> Result<String, String> {
    let mappings = vec![
        ("Int8", "i8"),
        ("Int32", "i16"),
        ("Int32", "i32"),
        ("Int64", "i64"),
        ("Int128", "i128"),
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

fn get_struct_def(parsed: &ParsedStruct) -> String {
    let gen_args = get_struct_gen_args(&parsed.generic_arguments);
    let gen_args_str = if gen_args.is_empty() {
        "".to_string()
    } else {
        let csv = gen_args.join(", ");
        format!("<{}>", csv)
    };

    return format!("{}{}", parsed.struct_name, gen_args_str);
}

pub fn generate_rust_struct(parsed: &ParsedStruct) -> ReturnT {
    let gen_args = get_struct_gen_args(&parsed.generic_arguments);

    let spacing = "    ";
    let name = get_struct_def(parsed);
    let struct_def = format!("pub struct {} {{", name);

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

    let use_common_line =
        r#"
use crate::{ 
    common_deserialize::{ deserialize_option, deserialize_scalar, deserialize_vec },
    common_serialize::{ serialize_option, serialize_scalar, serialize_vec },
    types::BufferT,
};"#.to_string();

    let mut lines = Vec::new();
    lines.push(use_common_line);
    lines.push("\n".to_string());

    lines.push("#[derive(Debug, Eq, PartialEq)]".to_string());
    lines.push(struct_def);
    lines.extend(field_lines);

    lines.push("}".to_string());
    lines.push("\n".to_string());
    return Ok(lines);
}

fn generate_serialize_line(field: &ParsedField) -> String {
    let s = match field.field_type.get_type() {
        AvailTypes::Scalar => "serialize_scalar",
        AvailTypes::Option => "serialize_option",
        AvailTypes::Union => todo!(),
        AvailTypes::Vec => "serialize_vec",
    };

    let line = format!(
        "        pos = {}(&self.{}, buffer, pos);",
        s,
        camel_to_snake(&field.field_name)
    );
    return line;
}

pub fn generate_rust_serialize_lines(parsed: &ParsedStruct) -> ReturnT {
    let fn_def = "    pub fn serialize_into(&self, buffer: &mut BufferT, pos: usize) -> usize {";
    let top = "        let mut pos: usize = pos;";
    let field_lines = parsed.fields
        .iter()
        .map(|f| generate_serialize_line(f))
        .collect::<Vec<String>>();

    let ret = "        return pos;";
    let fn_end = "    }";

    let mut lines = Vec::new();
    lines.push(fn_def.to_string());
    lines.push(top.to_string());
    lines.extend(field_lines);
    lines.push(ret.to_string());
    lines.push(fn_end.to_string());
    return Ok(lines);
}

fn generate_deserialize_line(parsed: &ParsedStruct, field: &ParsedField) -> String {
    let gen_args = get_struct_gen_args(&parsed.generic_arguments);

    let s = match field.field_type.get_type() {
        AvailTypes::Scalar =>
            format!(
                "deserialize_scalar::<{}>",
                get_mapped_type(&field.field_type.name, &gen_args).unwrap()
            ),
        AvailTypes::Option =>
            format!(
                "deserialize_option::<{}>",
                get_generic_arg_str(field.field_type.generic_args[0].as_ref(), &gen_args).unwrap()
            ),
        AvailTypes::Union => todo!(),
        AvailTypes::Vec =>
            format!(
                "deserialize_vec::<{}>",
                get_generic_arg_str(field.field_type.generic_args[0].as_ref(), &gen_args).unwrap()
            ),
    };

    // let arg = get_generic_arg_str(&field.field_type, &gen_args).unwrap();

    let additional = match field.field_type.get_type() {
        AvailTypes::Vec => { ".to_vec()" }
        _ => { "" }
    };

    let line = format!(
        "            {}: {}(&buffer, &mut pos){}, ",
        camel_to_snake(&field.field_name),
        s,
        additional
    );
    return line;
}

pub fn generate_rust_deserialize_lines(parsed: &ParsedStruct) -> ReturnT {
    let name = get_struct_def(parsed);

    let ret_typ = format!("Result<({}, usize), String>", name);
    let fn_def =
        format!("    pub fn deserialize_from(buffer: &BufferT, pos: usize) -> {} {{", ret_typ);

    let top = "        let mut pos: usize = pos;";
    let create = format!("        let obj = {} {{", name);

    let field_lines = parsed.fields
        .iter()
        .map(|f| generate_deserialize_line(parsed, f))
        .collect::<Vec<String>>();

    let obj_end = "        };";

    let ret = "        return Ok((obj, pos));";
    let fn_end = "    }";

    let mut lines = Vec::new();
    lines.push(fn_def.to_string());
    lines.push(top.to_string());
    lines.push(create);
    lines.extend(field_lines);
    lines.push(obj_end.to_string());
    lines.push(ret.to_string());
    lines.push(fn_end.to_string());
    return Ok(lines);
}

pub fn generate_rust_struct_full(parsed: &ParsedStruct) -> ReturnT {
    let mut to_output: Vec<String> = Vec::new();
    let gen_def = generate_rust_struct(parsed);
    let gen_ser = generate_rust_serialize_lines(parsed);
    let gen_deser = generate_rust_deserialize_lines(parsed);

    match (&gen_def, &gen_ser, &gen_deser) {
        (Ok(def), Ok(ser), Ok(dser)) => {
            to_output.extend(def.clone());
            to_output.push("\r".to_string());
            to_output.push(format!("impl {} {{", parsed.struct_name));
            to_output.extend(ser.clone());

            to_output.extend(dser.clone());

            to_output.push("}".to_string());
            return Ok(to_output);
        }
        _ => {
            if gen_def.is_err() {
                return Err(gen_def.unwrap_err());
            }
            if gen_ser.is_err() {
                return Err(gen_ser.unwrap_err());
            }
            if gen_deser.is_err() {
                return Err(gen_deser.unwrap_err());
            }

            return Err("Error generating struct".to_string());
        }
    }
}
