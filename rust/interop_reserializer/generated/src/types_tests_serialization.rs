use crate::{
    common_deserialize::{ deserialize_option, deserialize_scalar, deserialize_vec },
    types::OptionIdType,
};

pub struct BookUpdateBuffer<'a> {
    buffer: &'a [u8],
    offset: usize,
}
impl<'a> BookUpdateBuffer<'a> {
    // NOTE: The ASK_OFFSET_ID is the position that the AskOffset will be serialized into
    //       this will be right at the start of the buffer.
    pub const ASK_OFFSET_ID: usize = 0;

    pub const TIME_OFFSET: usize = Self::ASK_OFFSET_ID + size_of::<usize>();
    pub const TIME_EXCH_OFFSET: usize = Self::TIME_OFFSET + size_of::<i8>();
    pub const INST_ID_OFFSET: usize =
        Self::TIME_EXCH_OFFSET + size_of::<i32>() + size_of::<OptionIdType>();
    pub const UPDATE_TYPE_OFFSET: usize = Self::INST_ID_OFFSET + size_of::<i64>();
    pub const BIDS_OFFSET: usize = Self::UPDATE_TYPE_OFFSET + size_of::<i128>();

    pub fn from_buffer(buffer: &[u8], offset: usize) -> BookUpdateBuffer {
        BookUpdateBuffer {
            buffer,
            offset,
        }
    }

    // NOTE:
    //    asks offset is dependent on the size of BidsOffset + Vec.len()
    //    which is unknown until serialization time
    pub fn time(&self) -> i8 {
        let mut offset = self.offset + Self::TIME_OFFSET;
        return deserialize_scalar(self.buffer, &mut offset);
    }

    pub fn time_exch(&self) -> Option<i32> {
        let mut offset = self.offset + Self::TIME_EXCH_OFFSET;
        return deserialize_option(self.buffer, &mut offset);
    }

    pub fn inst_id(&self) -> i64 {
        let mut offset = self.offset + Self::INST_ID_OFFSET;
        return deserialize_scalar(self.buffer, &mut offset);
    }

    pub fn update_type(&self) -> i128 {
        let mut offset = self.offset + Self::UPDATE_TYPE_OFFSET;
        return deserialize_scalar(self.buffer, &mut offset);
    }

    pub fn bid_len(&self) -> usize {
        let mut offset = self.offset + Self::BIDS_OFFSET;
        return deserialize_scalar(self.buffer, &mut offset);
    }

    pub fn ask_len(&self) -> usize {
        let mut offset = Self::ASK_OFFSET_ID;
        let mut actual_pos = deserialize_scalar(self.buffer, &mut offset);
        return deserialize_scalar(self.buffer, &mut actual_pos);
    }

    pub fn bids(&self) -> &'a [i32] {
        let mut offset = self.offset + Self::BIDS_OFFSET;
        return deserialize_vec(self.buffer, &mut offset);
    }

    // TODO:  We need to get hold of the Asks offset
    pub fn asks(&self) -> &'a [i64] {
        let mut offset = Self::ASK_OFFSET_ID;
        let mut actual_pos = deserialize_scalar(self.buffer, &mut offset);
        return deserialize_vec(self.buffer, &mut actual_pos);
    }
}
