use crate::{ common_deserialize::deserialize_vec, common_serialize::serialize_vec };

pub trait BookUpdatePoc {
    fn bids(&self) -> &[i64];
    fn asks(&self) -> &[i64];
}

pub struct BookUpdatePocCreate {
    pub bids: Vec<i64>,
    pub asks: Vec<i64>,
}
impl BookUpdatePoc for BookUpdatePocCreate {
    fn bids(&self) -> &[i64] {
        return &self.bids;
    }

    fn asks(&self) -> &[i64] {
        return &self.asks;
    }
}
impl BookUpdatePocCreate {
    pub fn write_to_buffer(&self, buffer: &mut [u8], offset: usize) -> usize {
        let mut pos = offset;
        pos = serialize_vec(&self.bids, buffer, pos);
        pos = serialize_vec(&self.asks, buffer, pos);
        return pos;
    }
}

pub struct BookUpdatePocRead<'a> {
    buffer: &'a [u8],
    offset: usize,
}
impl<'a> BookUpdatePocRead<'a> {
    pub fn from_buffer(buffer: &[u8], offset: usize) -> BookUpdatePocRead {
        BookUpdatePocRead {
            buffer,
            offset,
        }
    }
}
impl<'a> BookUpdatePoc for BookUpdatePocRead<'a> {
    fn bids(&self) -> &[i64] {
        let mut offset = self.offset;
        return deserialize_vec::<i64>(&self.buffer, &mut offset);
    }

    fn asks(&self) -> &[i64] {
        let mut offset = self.offset;
        let _bids = deserialize_vec::<i64>(&self.buffer, &mut offset);
        return deserialize_vec::<i64>(&self.buffer, &mut offset);
    }
}
