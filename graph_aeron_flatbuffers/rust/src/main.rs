extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]

#[path = "../generated/common_generated.rs"]
pub mod common_generated;
pub use common_generated::graph::*;

#[path = "../generated/book_generated.rs"]
mod book_generated;
pub use book_generated::graph as book_graph;

mod flatbufferutils;
use flatbufferutils::{ create_book, process_book };


fn main() {
    let buffer = create_book::as_buffer();

    process_book::from_buffer(&buffer);
    
    println!("Hello, world!");
}
