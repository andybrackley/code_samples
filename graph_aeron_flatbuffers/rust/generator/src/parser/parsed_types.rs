#[derive(Debug, Clone)]
pub enum ParsedVariableType {
    Scaler(String),
    Generic(String, Vec<Box<ParsedVariableType>>)
}

impl ParsedVariableType { 
    pub fn compare(&self, rhs: &ParsedVariableType) -> bool {
        match (self, rhs) {
            (ParsedVariableType::Scaler(n), ParsedVariableType::Scaler(r_name)) => n == r_name,
            (ParsedVariableType::Generic(n, typ), ParsedVariableType::Generic(r_name, r_type )) => {
                if n != r_name { return false; }
                if typ.len() != r_type.len() { return false; }

                for(t, rt) in typ.iter().zip(r_type.iter()) {
                    if !t.compare(rt) {
                        return false;
                    }
                }

                return true
            }
            _ => false
        } 
    }
}

#[derive(Debug, Clone)]
pub struct ParsedField {
    pub field_name: String,
    pub field_type: ParsedVariableType
}

