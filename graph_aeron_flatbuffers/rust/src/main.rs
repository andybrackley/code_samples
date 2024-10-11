
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]

#[path = "../generated/common_generated.rs"]
pub mod common_generated;
use core::slice;
use std::{sync::{atomic::AtomicBool, Arc, Mutex}, thread};

use aeron::{concurrent::{atomic_buffer::{AlignedBuffer, AtomicBuffer}, logbuffer::header::Header }, publication::Publication, subscription::Subscription, utils::types::Index};
use aeronimpl::aeron_settings::{connection, publisher, subscriber, Settings};
pub use common_generated::graph::*;

#[path = "../generated/book_generated.rs"]
mod book_generated;
pub use book_generated::graph as book_graph;

mod flatbufferutils;
use flatbufferutils::{ create_book, process_book };

mod aeronimpl;

fn print_message_wrapper(is_running: Arc<AtomicBool>) -> impl Fn(&AtomicBuffer, Index, Index, &Header) {
    let is_running = is_running.clone();

    move | buffer: &AtomicBuffer, offset: Index, length: Index, _header: &Header | {
        unsafe {
            let slice = slice::from_raw_parts_mut(buffer.buffer().offset(offset as isize), length as usize);
            process_book::from_buffer(&slice.to_vec());
            is_running.store(false, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

fn run_subscriber(subscriber: &Arc<Mutex<Subscription>>, is_running: Arc<AtomicBool>) {
    let mut print_frag = print_message_wrapper(is_running.clone());
    let mut fragment_assembler = aeron::fragment_assembler::FragmentAssembler::new(&mut print_frag, None);
    let mut fragment_handler = fragment_assembler.handler();

    subscriber::handle(&subscriber, &mut fragment_handler, is_running.clone());
}
    

fn run_publisher(publisher: &Arc<Mutex<Publication>>, is_running: &AtomicBool) {
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


fn main_aeron_flatbuffers() {
    let settings = Settings::new(String::from("C:\\Users\\andyb\\AppData\\Local\\Temp\\aeron-andyb"));
    let connection_result = connection::connect(&settings);

    match connection_result {
        Ok(mut connection) => {
            let running_raw = Arc::new(AtomicBool::from(true));
            let running_pub = running_raw.clone();
            let running_sub = running_raw.clone();

            println!("Aeron connection setup, running publish/subscribe");
            let publisher = publisher::create(&mut connection, &settings);
            let subscriber = subscriber::create(&mut connection, &settings);
                    
            let publisher_thread = thread::spawn(move|| {run_publisher(&publisher, &running_pub)});
            let sub_thread = thread::spawn(move|| {run_subscriber(&subscriber, running_sub)} );
        
            let _ = publisher_thread.join();
            let _ = sub_thread.join();
        },

        Err(error) => {
            println!("Creating connection failed with error: {:?}", error);
        }
    }
    
    println!("Completed Publish/Subscribe");
}

fn main() {
    
}