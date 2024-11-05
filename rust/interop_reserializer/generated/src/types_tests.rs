use crate::{
    common_deserialize::{ deserialize_option, deserialize_scalar, deserialize_vec },
    common_serialize::{ serialize_option, serialize_scalar, serialize_vec },
    types::BufferT,
};

#[derive(Debug, Eq, PartialEq)]
pub struct BookUpdate {
    pub time: i8,
    pub timestamp_exch: Option<i32>,
    pub inst_id: i64,
    pub update_type: i128,
    pub bids: Vec<i32>,
    pub asks: Vec<i64>,
}

impl BookUpdate {
    pub fn serialize_into(&self, buffer: &mut BufferT, start_pos: usize) -> usize {
        // We need to store the position of the asks vector
        let mut pos: usize = start_pos + size_of::<usize>();
        // let mut pos = start_pos;

        pos = serialize_scalar(&self.time, buffer, pos);
        pos = serialize_option(&self.timestamp_exch, buffer, pos);
        pos = serialize_scalar(&self.inst_id, buffer, pos);
        pos = serialize_scalar(&self.update_type, buffer, pos);
        pos = serialize_vec(&self.bids, buffer, pos);
        // Serialize the position of the asks vector
        _ = serialize_scalar(&pos, buffer, start_pos);
        pos = serialize_vec(&self.asks, buffer, pos);
        return pos;
    }
    pub fn deserialize_from(buffer: &BufferT, pos: usize) -> Result<(BookUpdate, usize), String> {
        let mut pos: usize = pos;
        let _ask_index = deserialize_scalar::<usize>(buffer, &mut pos);

        let obj = BookUpdate {
            time: deserialize_scalar::<i8>(&buffer, &mut pos),
            timestamp_exch: deserialize_option::<i32>(&buffer, &mut pos),
            inst_id: deserialize_scalar::<i64>(&buffer, &mut pos),
            update_type: deserialize_scalar::<i128>(&buffer, &mut pos),
            bids: deserialize_vec::<i32>(&buffer, &mut pos).to_vec(),
            asks: deserialize_vec::<i64>(&buffer, &mut pos).to_vec(),
        };
        return Ok((obj, pos));
    }
}
