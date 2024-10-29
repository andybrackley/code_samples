use std::rc::Rc;

pub enum ParsedErrorTypes {
    NotOfType,
    Malformed(String)
}

trait Scoped {
    fn process_line(&self, line: &str) -> ScopeResult; 
}

struct StructTrait {}

impl Scoped for StructTrait {
    fn process_line(&self, line: &str) -> ScopeResult {
        return ScopeResult::CloseScope();
    }
}

enum Types {

}

struct GloablScope {
    defs: Vec<Types>
}

impl Scoped for GloablScope {
    fn process_line(&self, line: &str) -> ScopeResult {
        return ScopeResult::CloseScope();
    }
}



pub enum ScopeResult {
    Error(String),
    SameScope(),
    NewScope(Scope),
    CloseScope()
}

#[derive(Debug)]
pub enum ScopeType {
    Global(),
    Namespace(),
    Struct(),
    Enum(),
}

fn struct_scope() {}

#[derive(Debug)]
pub struct Scope {
    pub scope_type: ScopeType,
    pub parent: Option<Rc<Scope>>,
    pub children: Vec<Rc<Scope>>,
}

impl Scope {
    pub fn new(scope_type: ScopeType) -> Scope {
        Scope {
            scope_type,
            parent: None,
            children: Vec::new(),
        }
    }

    pub fn create_child_scope(mut parent: &Rc<Scope>, scope_type: ScopeType) -> Rc<Scope> {
        let child = Scope {
            scope_type,
            parent: Some(parent.clone()),
            children: Vec::new(),
        };

        let childrc = Rc::new(child);
        Rc::get_mut(&mut Rc::clone(&parent))
            .expect("Failed to get a mutable reference")
            .children
            .push(Rc::clone(&childrc));

        return childrc;
    }

    pub fn process_line(&self, line: &str) -> ScopeResult {
        ScopeResult::SameScope()
    }
}

