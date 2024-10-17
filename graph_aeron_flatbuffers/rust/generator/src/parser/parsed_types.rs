// #[derive(Debug, Clone)]
// pub enum ParsedVariableType {
//     Scaler(String),
//     Generic(String, Vec<Box<ParsedVariableType>>)
// }

// impl ParsedVariableType { 
//     pub fn compare(&self, rhs: &ParsedVariableType) -> bool {
//         match (self, rhs) {
//             (ParsedVariableType::Scaler(n), ParsedVariableType::Scaler(r_name)) => n == r_name,
//             (ParsedVariableType::Generic(n, typ), ParsedVariableType::Generic(r_name, r_type )) => {
//                 if n != r_name { return false; }
//                 if typ.len() != r_type.len() { return false; }

//                 for(t, rt) in typ.iter().zip(r_type.iter()) {
//                     if !t.compare(rt) {
//                         return false;
//                     }
//                 }

//                 return true
//             }
//             _ => false
//         } 
//     }
// }

#[derive(Debug, Clone)]
pub struct ParsedVariableType {
    pub name: String,
    pub generic_args: Vec<Box<ParsedVariableType>>
}
impl ParsedVariableType {
    pub fn Scaler(name: &str) -> ParsedVariableType { 
        ParsedVariableType {
            name: name.to_string(),
            generic_args: Vec::new()
        }
    }

    pub fn Generic(name: &str, type_args: Vec<Box<ParsedVariableType>>) -> ParsedVariableType {
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


#[derive(Debug, Clone)]
pub struct ParsedField {
    pub field_name: String,
    pub field_type: ParsedVariableType,
}
// #[derive(Debug, Clone)]
// pub struct GenericTypeArguments {

// }

#[derive(Debug, Clone)]
pub struct AbstractType {
    pub struct_name: String,
    pub generic_arguments: Vec<Box<ParsedVariableType>>
}

#[derive(Debug, Clone)]
pub struct ParsedStruct {
    pub is_mutable: bool,
    pub struct_name: String,
    pub fields: Vec<ParsedField>,
    pub inherits_from: Option<AbstractType>,
    pub generic_arguments: Vec<Box<ParsedVariableType>>
}
