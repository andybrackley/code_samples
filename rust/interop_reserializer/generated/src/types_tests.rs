use crate::{
    common_deserialize::deserialize_scalar,
    common_serialize::{ serialize_option, serialize_scalar, serialize_i64, serialize_vec },
    types::BufferT,
};

#[derive(Debug)]
pub struct BookUpdate {
    pub time: i64,
    pub timestamp_exch: Option<i64>,
    pub inst_id: i64,
    pub update_type: i64,
    pub bids: Vec<i64>,
    pub asks: Vec<i64>,
}
impl BookUpdate {
    pub fn serialize_into(&self, buffer: &mut BufferT, pos: usize) -> usize {
        let mut pos = serialize_i64(&self.time, buffer, pos);
        pos = serialize_option(&self.timestamp_exch, buffer, pos);
        pos = serialize_i64(&self.inst_id, buffer, pos);
        pos = serialize_i64(&self.update_type, buffer, pos);
        pos = serialize_vec(&self.bids, buffer, pos);
        pos = serialize_vec(&self.asks, buffer, pos);
        return pos;
    }

    pub fn deserialize_from(buffer: &BufferT, pos: usize) -> Result<(BookUpdate, usize), String> {
        let mut pos = pos;
        //  = deserialize_scalar(scalar, buffer, pos);
        return Err("".to_string());
    }
}
