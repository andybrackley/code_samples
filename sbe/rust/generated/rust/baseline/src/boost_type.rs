#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BoostType {
    TURBO = 84_u8, 
    SUPERCHARGER = 83_u8, 
    NITROUS = 78_u8, 
    KERS = 75_u8, 
    #[default]
    NullVal = 0_u8, 
}
impl From<u8> for BoostType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            84_u8 => Self::TURBO, 
            83_u8 => Self::SUPERCHARGER, 
            78_u8 => Self::NITROUS, 
            75_u8 => Self::KERS, 
            _ => Self::NullVal,
        }
    }
}
