#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct u2(u8);
impl u2{
    pub const ZERO: u2 = u2::from_low(0b00000000);
    pub const ONE: u2 = u2::from_low(0b00000001);
    pub const TOW: u2 = u2::from_low(0b00000010);
    pub const THREE: u2 = u2::from_low(0b00000011);

    #[inline]
    pub const fn from_low(val: u8) -> Self {
        Self(val & 0b00000011)
    }
}
