#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Model {
    A = 65_u8, 
    B = 66_u8, 
    C = 67_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for Model {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            65_u8 => Self::A, 
            66_u8 => Self::B, 
            67_u8 => Self::C, 
            _ => Self::NullVal,
        }
    }
}
