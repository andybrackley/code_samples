use std::collections::{btree_set::Union, HashMap, HashSet};

use Helpers::create_union;

use super::parsed_types::{ParsedField, ParsedVariableType};


// struct Test {
//     multi_generics: MultiGenericsType
// }

const OPEN_GENERIC_CHAR: &str = "<"; 
const CLOSE_GENERIC_CHAR: &str = ">"; 

struct EnvState {
    builtin_types: HashMap<String, String>,
    known_types:   HashSet<String>
}
impl EnvState {
    pub fn for_rust() -> EnvState {
        // TODO: Should be constant
        let mut type_lookup = HashMap::new();
        type_lookup.insert("Int64".to_string(), "i64".to_string());
        type_lookup.insert("TimeStamp".to_string(), "i32".to_string());
        type_lookup.insert("String".to_string(), "String".to_string());
        type_lookup.insert("Vector".to_string(), "Vec".to_string());
        type_lookup.insert("Union".to_string(), "Union".to_string());
        
        let mut known_types = HashSet::new();
        EnvState {
            builtin_types: type_lookup,
            known_types: known_types
        } 
    }

    pub fn get_mapped(&self, julia_type: &str) -> String {
        let ty = self.builtin_types.get(julia_type);
        match ty {
            Some(t) => format!("{}", t),
            None => {
                let kt = self.known_types.get(julia_type);
                match kt {
                    Some(t) => format!("{}", t),
                    None => format!("ERROR Looking up Type: '{};", julia_type)
                }
            }
        }                

    }
}

mod Helpers {
    use crate::parser::{parsed_types::ParsedVariableType, types_outputter::Helpers};
    use super::EnvState;

    fn to_camel_case(s: &str) -> String {
        let mut camel_case = String::new();
        let mut upper_next = true;
        for c in s.chars() {
            if c == '_' {
                upper_next = true;
            } else if upper_next {
                camel_case.push(c.to_ascii_uppercase());
                upper_next = false;
            } else {
                camel_case.push(c);
            }
        }
        camel_case
    }

    pub fn get_union_name(field_name: &str, types: &Vec<Box<ParsedVariableType>>) -> String {
        // TODO:  Need a sensible way of creating the name of the enum
        format!("{}", to_camel_case(field_name))
    }

    pub fn create_union(env: &EnvState, field_name: &str, types: &Vec<Box<ParsedVariableType>>) -> String {
        println!("Creating union for: {}", field_name);
        
        let name = get_union_name(field_name, types);
        let mut path = name.clone();

        println!("pub enum {} {{", name);

        for t in types {
            match t.as_ref() {
                ParsedVariableType::Scaler(s) => {
                    let n = env.get_mapped(s);
                    path.push_str(&s);

                    println!("   As{n}({n}),");
                },

                // TODO: For the AsVec() implementation it also needs ALL the generic args
                ParsedVariableType::Generic(s, ty) => {
                    path.push_str(s);
                    println!("CreateUnion::{}.{}", path, s);

                    let n = env.get_mapped(s);
                    println!("   As{n}({n}),");
                },
            }
        }
        
        println!("}}");
        return name;
    }
}



pub fn output(field: ParsedField) {
    let mut env_state = EnvState::for_rust();
    env_state.known_types.insert("BookUpdateType".to_string());

    fn print_field_type(field_name: &str, field_type: &ParsedVariableType, env: &EnvState) -> String {
        match field_type {
            ParsedVariableType::Scaler(type_name) => {
                let type_name = env.get_mapped(type_name);
                format!("{}", type_name)
            },
            ParsedVariableType::Generic(type_name, types) => {
                let inner_types = types.into_iter().map(|t| print_field_type(format!("{}{}", field_name, type_name).as_str(), t, &env)).collect::<Vec<String>>().join(", ");
                let type_name = env.get_mapped(type_name);

                let is_union = type_name == "Union";
                if(is_union) {
                    let union_name = create_union(env, field_name, &types);
                }

                if inner_types.is_empty() {
                    return format!("{}", type_name)
                } else {
                    format!("{}<{}>", type_name, inner_types)
                }
            }
        }
    }

    println!("Rust: {}: {}", field.field_name, print_field_type(field.field_name.as_str(), &field.field_type, &env_state));
}