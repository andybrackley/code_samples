use std::{collections::HashSet, rc::Rc};

use super::parser_types::ParsedField;

#[derive(Debug, Clone)]
pub struct FieldPositions {
    pub original_order: Vec<Rc<ParsedField>>,
    pub fixed_size_fields: Vec<Rc<ParsedField>>,
    pub var_sized_fields: Vec<Rc<ParsedField>>,
}
impl FieldPositions {
    pub fn create_from_list(
        fields: &Vec<Rc<ParsedField>>,
        var_sized_types: &HashSet<String>,
    ) -> Self {
        let mut fixed_size_fields = Vec::new();
        let mut var_sized_fields = Vec::new();
        let mut original_order = Vec::new();

        for field in fields {
            let types = field.field_type.flatten();
            let is_fixed_size = types.iter().any(|t| var_sized_types.contains(t));

            original_order.push(field.clone());

            if is_fixed_size {
                var_sized_fields.push(field.clone());
            } else {
                fixed_size_fields.push(field.clone());
            }
        }

        FieldPositions {
            original_order,
            fixed_size_fields,
            var_sized_fields,
        }
    }

    pub fn non_offset_fields(&self) -> Vec<Rc<ParsedField>> {
        let mut fields = Vec::new();
        fields.extend(self.fixed_size_fields.iter().cloned());
        fields.extend(self.var_sized_fields.iter().take(1).cloned());
        fields
    }

    pub fn offset_fields(&self) -> Vec<Rc<ParsedField>> {
        let mut fields = Vec::new();
        fields.extend(self.var_sized_fields.iter().skip(1).cloned());
        fields
    }

    pub fn in_serialize_order(&self) -> Vec<Rc<ParsedField>> {
        let mut fields = Vec::new();
        fields.extend(self.fixed_size_fields.iter().cloned());
        fields.extend(self.var_sized_fields.iter().cloned());
        fields
    }

    pub fn is_var_sized(&self) -> bool {
        !self.var_sized_fields.is_empty()
    }

    pub fn is_field_offset(&self, field: &Rc<ParsedField>) -> bool {
        self.offset_fields().contains(field)
    }

    pub fn is_array_type(&self, field: &Rc<ParsedField>) -> bool {
        match field.field_type.name.as_str() {
            "Array" => true,
            "Vector" => true,
            _ => false,
        }
    }

    pub fn is_union_type(&self, field: &Rc<ParsedField>) -> bool {
        match field.field_type.name.as_str() {
            "Union" => true,
            "Optional" => true,
            _ => false,
        }
    }
}
