//pub mod v2;
pub mod v3;
pub use bobbin_bits;
use bobbin_bits::U4;

pub enum InstructionSet {
    V2,
    V3,
}
