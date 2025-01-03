pub struct Constants {}

pub type OptionIdType = i8;

impl Constants {
    pub const OPTION_NONE: OptionIdType = 0;
    pub const OPTION_SOME: OptionIdType = 1;
}

pub type BufferT = Vec<u8>;

// This aligns vector onto 8 byte boundaries
// TODO: Is there a better way of doing this....
pub fn calc_padding(pos: usize) -> usize {
    let r = pos % 8;
    let to_pad = if r == 0 { 0 } else { 8 - r };
    to_pad
}
