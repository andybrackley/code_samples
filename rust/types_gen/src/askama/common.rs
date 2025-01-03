use crate::common::parser_types::ParsedStruct;

pub struct Field {
    pub field: String,
    pub typ: String,
}

pub struct StructDefDetails {
    pub struct_name: String,
    pub generic_args: String,
    pub fields: Vec<Field>,
}
impl StructDefDetails {
    pub fn from_parsed(def: &ParsedStruct) -> Self {
        let args: Vec<String> = Vec::new();

        Self {
            struct_name: def.struct_name.clone(),
            generic_args: args.join(", "),
            fields: def.fields
                .iter()
                .map(|f| Field { field: f.field_name.clone(), typ: f.field_type.name.clone() })
                .collect(),
        }
    }
}
