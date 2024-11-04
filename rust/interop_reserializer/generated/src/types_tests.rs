use crate::{ common_serialize::{ serialize_option, serialize_scalar, serialize_vec }, types::BufferT };


#[derive(Debug, Eq, PartialEq)]
pub struct BookUpdate {
    pub time: i8,
    pub timestamp_exch: Option<i32>,
    pub inst_id: i64,
    pub update_type: i128,
    pub bids: Vec<i32>,
    pub asks: Vec<i64>,
}


