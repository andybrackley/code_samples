use std::{collections::HashSet, rc::Rc};

use crate::common::{
    field_positions::{self, FieldPositions},
    parser_types::{EnumType, ParsedField, ParsedStruct, ParsedVariableType},
};

#[derive(Debug, Clone)]
pub struct EnumValueDetails {
    pub value_name: String,
    pub value: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct EnumDefDetails {
    pub enum_name: String,
    pub values: Vec<EnumValueDetails>,
}
impl EnumDefDetails {
    pub fn from_parsed(def: &EnumType) -> Self {
        Self {
            enum_name: def.name.clone(),
            values: def
                .values
                .iter()
                .map(|v| EnumValueDetails {
                    value_name: v.name.clone(),
                    value: v.value,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StructDefDetails {
    pub struct_name: String,
    pub is_mutable: bool,
    pub generic_args: Vec<Box<ParsedVariableType>>,

    pub field_positions: FieldPositions,
}
impl StructDefDetails {
    pub fn from_parsed(def: &ParsedStruct, var_sized_types: &HashSet<String>) -> Self {
        let field_positions = FieldPositions::create_from_list(
            &def.fields.iter().map(|f| Rc::new(f.clone())).collect(),
            var_sized_types,
        );

        Self {
            struct_name: def.struct_name.clone(),
            is_mutable: def.is_mutable,
            generic_args: def.generic_arguments.clone(),
            field_positions: field_positions.clone(),
        }
    }

    pub fn is_var_sized(&self) -> bool {
        self.field_positions.is_var_sized()
    }

    pub fn is_fixed_size(&self) -> bool {
        !self.is_var_sized()
    }

    pub fn offset_count(&self) -> usize {
        self.field_positions.offset_fields().len()
    }

    pub fn slot_count(&self) -> usize {
        if self.is_fixed_size() {
            0
        } else {
            self.offset_count() + 1
        }
    }

    pub fn field_order_orig(&self) -> &Vec<Rc<ParsedField>> {
        &self.field_positions.original_order
    }

    pub fn field_order_ser(&self) -> Vec<Rc<ParsedField>> {
        self.field_positions.in_serialize_order()
    }
}
