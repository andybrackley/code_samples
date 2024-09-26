extern crate flatbuffers;

#[path = "../../generated/common_generated.rs"]
mod common_generated;

#[path = "../../generated/book_generated.rs"]
mod book_generated;

// pub mod flat_buffer_utils {
    use crate::{ InstrumentId, InstrumentIdArgs, Exchange, Timestamp };
    use crate::{ book_generated::graph::BookUpdateBuilder };

    pub fn as_buffer() -> Vec<u8> {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let inst_id_str = builder.create_string("InstId1");
        
        let inst_id_args = InstrumentIdArgs {
            exchange: Exchange::Deribit,
            id: Some(inst_id_str)
        };
    
        let inst_id = InstrumentId::create(&mut builder, &inst_id_args);
        let timestamp = Timestamp::new(100);
    
        let mut bub = BookUpdateBuilder::new(&mut builder);
        bub.add_timestamp(&timestamp);
        bub.add_id(inst_id);
        let offset = bub.finish();
        let pointer = builder.finish(offset, None);
        let fin_buf = builder.finished_data();
        fin_buf.to_vec()
    }
// }