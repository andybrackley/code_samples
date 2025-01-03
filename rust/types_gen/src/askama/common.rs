use crate::common::parser_types::{ ParsedStruct, ParsedVariableType };

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
    pub fn from_parsed(
        def: &ParsedStruct,
        typ_formatter: &dyn Fn(&ParsedVariableType) -> String
    ) -> Self {
        let args: Vec<String> = def.generic_arguments
            .iter()
            .map(|a| typ_formatter(a))
            .collect();

        Self {
            struct_name: def.struct_name.clone(),
            generic_args: args.join(", "),
            fields: def.fields
                .iter()
                .map(|f| Field {
                    field: f.field_name.clone(),
                    typ: typ_formatter(&f.field_type),
                })
                .collect(),
        }
    }
}

pub trait VarTypeFormatter {
    fn format_var_type(&self, typ: &ParsedVariableType) -> String;
}
