use crate::{ common_serialize::{ serialize_option, serialize_scalar, serialize_vec }, types::BufferT };


#[derive(Debug, Eq, PartialEq)]
pub struct BookUpdate {
    pub time: i64,
    pub timestamp_exch: Option<i64>,
    pub inst_id: i64,
    pub update_type: i64,
    pub bids: Vec<i64>,
    pub asks: Vec<i64>,
}


