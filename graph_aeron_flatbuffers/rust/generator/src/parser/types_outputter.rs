use std::collections::btree_set::Union;

use super::parsed_types::{ParsedField, ParsedVariableType};

pub enum MultiGenericsTypeVec {
    AsInt(i64),
    AsTimestamp(i32),
    AsString(String) 
}

pub enum MultiGenericsType {
    AsVector(Vec<MultiGenericsTypeVec>),
    AsBookUpdateType(i8)
}

struct Test {
    multi_generics: MultiGenericsType
}

const OPEN_GENERIC_CHAR: &str = "<"; 
const CLOSE_GENERIC_CHAR: &str = ">"; 

pub fn output(field: ParsedField) {

    fn print_field_type(field_type: &ParsedVariableType) -> String {
        match field_type {
            ParsedVariableType::Scaler(type_name) => format!("{}", type_name),
            ParsedVariableType::Generic(type_name, types) => {
                let inner_types = types.into_iter().map(|t| print_field_type(t)).collect::<Vec<String>>().join(", ");
                if inner_types.is_empty() {
                    return format!("{}", type_name)
                } else {
                    format!("{}<{}>", type_name, inner_types)
                }
            }
        }
    
    }

    println!("Rust: {}: {}", field.field_name, print_field_type(&field.field_type));
}