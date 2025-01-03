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
            generic_args: type_args
                .into_iter()
                .map(|x| Box::new(x))
                .collect(),
        }
    }

    pub fn get_type(&self) -> AvailTypes {
        match self.name.as_str() {
            "Optional" => AvailTypes::Option,
            "Union" => AvailTypes::Union,
            "Vector" => AvailTypes::Vec,
            _ => AvailTypes::Scalar,
        }
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

#[derive(Debug, Clone)]
pub enum ParsedType {
    Enum(EnumType),
    Alias(AliasType),
    Abstract(AbstractType),
    Struct(ParsedStruct),
}
