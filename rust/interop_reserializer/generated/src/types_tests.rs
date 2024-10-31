use crate::{
    common_deserialize::{ deserialize_option, deserialize_scalar, deserialize_vec },
    common_serialize::{ serialize_option, serialize_scalar, serialize_vec },
    types::BufferT,
};

#[derive(Debug, Eq, PartialEq)]
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
        let mut pos = serialize_scalar(&self.time, buffer, pos);
        pos = serialize_option(&self.timestamp_exch, buffer, pos);
        pos = serialize_scalar(&self.inst_id, buffer, pos);
        pos = serialize_scalar(&self.update_type, buffer, pos);
        pos = serialize_vec(&self.bids, buffer, pos);
        pos = serialize_vec(&self.asks, buffer, pos);
        return pos;
    }

    pub fn deserialize_from(buffer: &BufferT, pos: usize) -> Result<(BookUpdate, usize), String> {
        let mut pos = pos;

        // TODO:  Can I do this without the derefence????
        let bu = BookUpdate {
            time: *deserialize_scalar::<i64>(&buffer, &mut pos),
            timestamp_exch: deserialize_option(&buffer, &mut pos).map(|o| *o),
            inst_id: *deserialize_scalar::<i64>(&buffer, &mut pos),
            update_type: *deserialize_scalar::<i64>(&buffer, &mut pos),
            bids: deserialize_vec::<i64>(&buffer, &mut pos)
                .iter()
                .map(|&v| *v)
                .collect(),

            asks: deserialize_vec::<i64>(&buffer, &mut pos)
                .iter()
                .map(|&v| *v)
                .collect(),
        };

        return Ok((bu, pos));
    }
}