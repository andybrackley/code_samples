use crate::common::parser_types::{ ParsedStruct, ParsedVariableType };

#[derive(Debug, Clone)]
pub struct Field {
    pub field: String,
    pub typ: ParsedVariableType,
    pub prev: Option<Box<Field>>,
}

#[derive(Debug, Clone)]
pub struct StructDefDetails {
    pub struct_name: String,
    pub is_mutable: bool,
    pub generic_args: Vec<Box<ParsedVariableType>>,
    pub fields: Vec<Field>,
}
impl StructDefDetails {
    pub fn from_parsed(def: &ParsedStruct) -> Self {
        let mut prev_field: Option<Box<Field>> = None;

        Self {
            struct_name: def.struct_name.clone(),
            is_mutable: def.is_mutable,
            generic_args: def.generic_arguments.clone(),
            fields: def.fields
                .iter()
                .map(|f| {
                    let field = Field {
                        field: f.field_name.clone(),
                        typ: f.field_type.clone(),
                        prev: prev_field.take(),
                    };

                    prev_field = Some(Box::new(field.clone()));
                    field
                })
                .collect(),
        }
    }
}
