#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Const,
    Enum,
    Abstract,
    Type,
    Mutable,
    Struct,
    FieldSeparator /* '::' */,
    Begin,
    End /* "end"  */,
    OpenGeneric /* '{' */,
    CloseGeneric /* '}' */,
    InheritSymbol /* '<:' */,
    Comma,
    NewLine /* \n, \r */,
    Identifier(String),
    Equal,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line_number: u32,
    pub char_pos: u32,
    pub token_pos: u32,
    // pub line: String,
}
