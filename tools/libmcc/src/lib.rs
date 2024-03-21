//pub mod v2;
pub mod v3;
#[allow(non_camel_case_types)]
pub struct u4(u8);

impl u4 {
    pub fn from_low(val: u8) -> Self {
        Self(val & 0x0F)
    }
    pub fn from_high(val: u8) -> Self {
        Self::from_low(val >> 4)
    }
}

pub enum InstructionSet {
    V2,
    V3,
}
