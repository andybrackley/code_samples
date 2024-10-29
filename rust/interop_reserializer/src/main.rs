/*
   Rules:

    1. Each resource can only have one mutable reference or any number of immutable references at a time.
    2. References must always be valid, which means that the resource being referenced must remain in scope for the entire lifetime of the reference.
    3. A mutable reference cannot exist at the same time as any other reference, mutable or immutable.
*/

/*
    Lifetime Elision:

    1. Each parameter that is a reference gets its lifetime parameter. In other words, a function with one parameter of type &T would have a single lifetime parameter, such as fn foo<'a>(x: &'a T).
    2. If there is exactly one input lifetime parameter (ie, &self, &mut self, or &), that lifetime is assigned to all output lifetime parameters.
    3. If there are multiple input lifetime parameters but one of them is &self or &mut self, the lifetime of &self or &mut self is assigned to all output lifetime parameters.
*/

use interop_reserializer::{lexer::Lexer, lexer_types::TokenType};

fn main() {
    let to_parse = "";
    let parsed = Lexer::parse(&to_parse);
    let _tkn = TokenType::Abstract;
    println!("{:#?}", parsed);
}