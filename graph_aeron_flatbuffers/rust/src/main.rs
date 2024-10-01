extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]

#[path = "../generated/common_generated.rs"]
pub mod common_generated;
use std::sync::{Arc, Mutex};

use aeron::{concurrent::{atomic_buffer::{AlignedBuffer, AtomicBuffer}, logbuffer::header::Header }, publication::Publication, subscription::Subscription, utils::types::Index};
use aeronimpl::aeron_settings::{connection, publisher, subscriber, Settings};
pub use common_generated::graph::*;

#[path = "../generated/book_generated.rs"]
mod book_generated;
pub use book_generated::graph as book_graph;

mod flatbufferutils;
use flatbufferutils::{ create_book, process_book };

mod aeronimpl;

// lazy_static! {
//     pub static ref RUNNING: AtomicBool = AtomicBool::from(true);
// }

fn print_message(buffer: &AtomicBuffer, offset: Index, length: Index, header: &Header) {
    unsafe {
        println!("Received message");
    }
}

fn run_subscriber(subscriber: &Arc<Mutex<Subscription>>) {
    let mut print_frag = print_message;
    let mut fragment_assembler = aeron::fragment_assembler::FragmentAssembler::new(&mut print_frag, None);
    let mut fragment_handler = fragment_assembler.handler();

    subscriber::handle(&subscriber, &mut fragment_handler);
}
    

fn run_publisher(publisher: &Arc<Mutex<Publication>>) {
    let mut srcBuffer = create_book::as_buffer();
    process_book::from_buffer(&srcBuffer);

    let buffer = AlignedBuffer::with_capacity(256);
    let src_buffer = AtomicBuffer::from_aligned(&buffer);


    src_buffer.put_bytes(0, &srcBuffer);

    // publisher.lock().unwrap().try_claim(length, buffer_claim)

    let result = publisher.lock().unwrap().offer_part(src_buffer, 0, buffer.len);
    if let Ok(code) = result {
        println!("Published with code: {}", code);
    } else {
        println!("Publish failed with code: {:?}", result.err());
    }
}

fn main() {

    let settings = Settings::new();
    let connection_result = connection::connect(&settings);

    match connection_result {
        Ok(mut connection) => {
            let publisher = publisher::create(&mut connection, &settings);
            let subscriber = subscriber::create(&mut connection, &settings);
                    
            run_publisher(&publisher);
            run_subscriber(&subscriber);
        },

        Err(error) => {
            println!("Creating connection failed with error: {:?}", error);
        }
    }
    

    println!("Hello, world!");
}
