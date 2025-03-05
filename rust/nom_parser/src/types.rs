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
