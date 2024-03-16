//pub mod v2;
pub mod v3;
pub use bobbin_bits;
pub use bobbin_bits::U4;

pub trait U4Add {
    fn overflowing_add(self, other: U4) -> U4;
}
impl U4Add for U4 {
    fn overflowing_add(self, other: U4) -> U4 {
        (self.into_u8() + other.into_u8()).into()
    }
}

pub enum InstructionSet {
    V2,
    V3,
}
