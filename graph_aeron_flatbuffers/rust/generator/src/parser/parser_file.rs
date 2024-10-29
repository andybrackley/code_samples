// JuliaParser:
//      https://github.com/JuliaLang/JuliaSyntax.jl
//      https://github.com/JuliaLang/julia

use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, rc::{Rc, Weak}};

use super::parsed_scope::{Scope, ScopeResult, ScopeType};

pub fn parse_lines(lines: &Vec<&str>) -> Result<Scope, String> {
    let current_scope = Rc::new(Scope::new(ScopeType::Global()));
    let mut active_scope = current_scope.clone();

    for (line_number, line) in lines.iter().enumerate() {
        if line.is_empty() { continue; }

        let scope_result = active_scope.borrow_mut().process_line(line);

        match scope_result {
            ScopeResult::Error(err) => {
                return Err(err);
            },
            ScopeResult::SameScope() => {},
            ScopeResult::NewScope(new_scope_type) => {
                // let new_scope = Scope::create_child_scope(&active_scope, new_scope_type);
                // active_scope = Rc::clone(&new_scope);
                active_scope = Rc::new(new_scope_type);
            },
            ScopeResult::CloseScope() => {
                match &active_scope.parent {
                    Some(parent) => {
                        active_scope = parent.clone();
                    },
                    None => {
                        return Err("Unexpected end of scope".to_string());
                    }
                }
            },
        }
    }

    let return_val = Rc::try_unwrap(current_scope); 
    Ok(return_val.unwrap())
}

