use std::collections::HashSet;

pub enum AvailTypes {
    Scalar,
    Option,
    Union,
    Vec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedVariableType {
    pub name: String,
    pub generic_args: Vec<Box<ParsedVariableType>>,
}
impl ParsedVariableType {
    pub fn scalar(name: &str) -> ParsedVariableType {
        ParsedVariableType {
            name: name.to_string(),
            generic_args: Vec::new(),
        }
    }

    pub fn generic(name: &str, type_args: Vec<ParsedVariableType>) -> ParsedVariableType {
        ParsedVariableType {
            name: name.to_string(),
            generic_args: type_args.into_iter().map(|x| Box::new(x)).collect(),
        }
    }

    pub fn flatten(&self) -> HashSet<String> {
        let mut result = HashSet::new();
        result.insert(self.name.clone());

        for arg in &self.generic_args {
            result.extend(arg.flatten());
        }

        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedField {
    pub field_name: String,
    pub field_type: ParsedVariableType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    pub name: String,
    pub value: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<EnumValue>,
}

// Rust = type TypeName<T, U> = TypeImpl<T, U>
// C++  = template <typename T, typename U> using TestAlias = TypeImpl<T, U>
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasType {
    pub alias_type: ParsedVariableType,
    pub target_type: ParsedVariableType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractType {
    pub struct_name: String,
    pub generic_arguments: Vec<Box<ParsedVariableType>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedStruct {
    pub is_mutable: bool,
    pub struct_name: String,
    pub fields: Vec<ParsedField>,
    pub inherits_from: Option<AbstractType>,
    pub generic_arguments: Vec<Box<ParsedVariableType>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParsedType {
    Enum(EnumType),
    Alias(AliasType),
    Abstract(AbstractType),
    Struct(ParsedStruct),
}
impl ParsedType {
    pub fn get_type_name(&self) -> String {
        match self {
            ParsedType::Enum(enum_type) => enum_type.name.clone(),
            ParsedType::Alias(alias_type) => alias_type.alias_type.name.clone(),
            ParsedType::Abstract(abstract_type) => abstract_type.struct_name.clone(),
            ParsedType::Struct(parsed_struct) => parsed_struct.struct_name.clone(),
        }
    }

    pub fn get_all_types(&self) -> HashSet<String> {
        match self {
            ParsedType::Enum(enum_type) => {
                let mut result = HashSet::new();
                result.insert(enum_type.name.clone());
                result
            }
            ParsedType::Alias(alias_type) => {
                let mut result = HashSet::new();
                result.extend(alias_type.alias_type.flatten());
                result.extend(alias_type.target_type.flatten());
                result
            }
            ParsedType::Abstract(abstract_type) => {
                let mut result = HashSet::new();
                result.insert(abstract_type.struct_name.clone());
                for arg in &abstract_type.generic_arguments {
                    result.extend(arg.flatten());
                }
                result
            }
            ParsedType::Struct(parsed_struct) => {
                let mut result = HashSet::new();
                result.insert(parsed_struct.struct_name.clone());
                for arg in &parsed_struct.generic_arguments {
                    result.extend(arg.flatten());
                }
                for field in &parsed_struct.fields {
                    result.extend(field.field_type.flatten());
                }
                if let Some(abstract_type) = &parsed_struct.inherits_from {
                    result.insert(abstract_type.struct_name.clone());
                    for arg in &abstract_type.generic_arguments {
                        result.extend(arg.flatten());
                    }
                }
                result
            }
        }
    }
}
