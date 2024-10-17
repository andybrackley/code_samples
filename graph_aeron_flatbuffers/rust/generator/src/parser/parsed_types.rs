#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedVariableType {
    pub name: String,
    pub generic_args: Vec<Box<ParsedVariableType>>
}
impl ParsedVariableType {
    pub fn scaler(name: &str) -> ParsedVariableType { 
        ParsedVariableType {
            name: name.to_string(),
            generic_args: Vec::new()
        }
    }

    pub fn generic(name: &str, type_args: Vec<Box<ParsedVariableType>>) -> ParsedVariableType {
        ParsedVariableType {
            name: name.to_string(),
            generic_args: type_args
        }
    }

    pub fn compare(&self, rhs: &ParsedVariableType) -> bool {
        if self.name != rhs.name { return false; }
        if self.generic_args.len() != rhs.generic_args.len() { return false; }

        for(t, rt) in self.generic_args.iter().zip(rhs.generic_args.iter()) {
            if !t.compare(rt) {
                return false;
            }
        };

        return true;
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedField {
    pub field_name: String,
    pub field_type: ParsedVariableType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasType {
    pub alias_type: ParsedVariableType,
    pub target_type: ParsedVariableType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractType {
    pub struct_name: String,
    pub generic_arguments: Vec<Box<ParsedVariableType>>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedStruct {
    pub is_mutable: bool,
    pub struct_name: String,
    pub fields: Vec<ParsedField>,
    pub inherits_from: Option<AbstractType>,
    pub generic_arguments: Vec<Box<ParsedVariableType>>
}
