extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]

#[path = "../generated/common_generated.rs"]
pub mod common_generated;
pub use common_generated::graph::*;

#[path = "../generated/book_generated.rs"]
mod book_generated;
pub use book_generated::graph as book_graph;


// pub mod generated {
//     pub mod common_generated;
//     pub mod book_generated;
// } 

// pub use generated::common_generated::graph;
// pub use generated::book_generated::graph as book_graph;

// pub use common_generated::graph::*;
// pub use book_generated::graph_book;

fn main() {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let inst_id_str = builder.create_string("InstId1");
    
    let inst_id_args = InstrumentIdArgs {
        exchange: Exchange::Deribit,
        id: Some(inst_id_str)
    };

    let inst_id = InstrumentId::create(&mut builder, &inst_id_args);
    let timestamp = Timestamp::new(100);

    let mut bub = book_graph::BookUpdateBuilder::new(&mut builder);
    bub.add_timestamp(&timestamp);
    bub.add_id(inst_id);
    let offset = bub.finish();
    let pointer = builder.finish(offset, None);
    let fin_buf = builder.finished_data();


    let book_update = book_graph::root_as_book_update(&fin_buf);
    match book_update {
        Ok(upd) => {
             println!("Ok: {}", upd.id().unwrap().id().unwrap().chars().as_str());
        },
        Err(e) => {
            println!("{}", e);
        }
    };

    println!("Hello, world!");
}
