extern crate flatbuffers;

#[path = "../../generated/common_generated.rs"]
mod common_generated;

#[path = "../../generated/book_generated.rs"]
mod book_generated;
use book_generated::graph as book_graph;

// pub mod flat_buffer_utils {
    use crate::{ book_generated::graph::BookUpdateBuilder };

    pub fn from_buffer(buffer: &Vec<u8>) {
        let book_update = book_graph::root_as_book_update(&buffer);
        match book_update {
            Ok(upd) => {
                 println!("Ok: {}", upd.id().unwrap().id().unwrap().chars().as_str());
            },
            Err(e) => {
                println!("{}", e);
            }  
        };
}
// }