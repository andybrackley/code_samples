use std::collections::HashSet;

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
            generic_args: type_args
                .into_iter()
                .map(|x| Box::new(x))
                .collect(),
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
pub struct AliasType {
    pub alias_type: ParsedVariableType,
    pub target_type: ParsedVariableType,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractType {
    pub struct_name: String,
    pub generic_arguments: Vec<Box<ParsedVariableType>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedField {
    pub field_name: String,
    pub field_type: ParsedVariableType,
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
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParsedItem {
    Comment(String),
    Type(ParsedType),
}
