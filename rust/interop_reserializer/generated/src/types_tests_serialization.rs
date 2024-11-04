use crate::{
    common_deserialize::{ deserialize_scalar, deserialize_vec, deserialize_option },
    common_serialize::{ serialize_option, serialize_scalar, serialize_vec },
    types::BufferT,
    types_tests::BookUpdate,
};

impl BookUpdate {
    pub fn serialize_into2(&self, buffer: &mut BufferT, pos: usize) -> usize {
        let mut pos = serialize_scalar(&self.time, buffer, pos);
        pos = serialize_option(&self.timestamp_exch, buffer, pos);
        pos = serialize_scalar(&self.inst_id, buffer, pos);
        pos = serialize_scalar(&self.update_type, buffer, pos);
        pos = serialize_vec(&self.bids, buffer, pos);
        pos = serialize_vec(&self.asks, buffer, pos);
        return pos;
    }

    pub fn deserialize_from2(buffer: &BufferT, pos: usize) -> Result<(BookUpdate, usize), String> {
        let mut pos = pos;

        // TODO:  Can I do this without the derefence????
        let bu = BookUpdate {
            time: deserialize_scalar::<i8>(&buffer, &mut pos),
            timestamp_exch: deserialize_option::<i32>(&buffer, &mut pos).map(|o| o),
            inst_id: deserialize_scalar::<i64>(&buffer, &mut pos),
            update_type: deserialize_scalar::<i128>(&buffer, &mut pos),
            bids: deserialize_vec::<i32>(&buffer, &mut pos).to_vec(),
            asks: deserialize_vec::<i64>(&buffer, &mut pos).to_vec(),
        };

        return Ok((bu, pos));
    }
}
