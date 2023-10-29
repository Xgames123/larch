pub type Memory = [Bank; 16];

#[derive(Clone, Debug, Copy)]
pub struct Bank {
    pub data: [u8; 16],
}
impl From<[u8; 16]> for Bank {
    fn from(value: [u8; 16]) -> Self {
        Self { data: value }
    }
}
impl Default for Bank {
    fn default() -> Self {
        Self { data: [0; 16] }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    La,
    Sa,
    Lb,
    Sb,
    Lk,
    Sk,

    R,
    W,
    R2,
    W2,

    Jeq,
    Jeq2,
    Jmp,
    Jmp2,

    Add,
    Xor,
}
impl From<Instruction> for u8 {
    fn from(value: Instruction) -> Self {
        use Instruction::*;
        match value {
            La => 0x0,
            Sa => 0x1,
            Lb => 0x2,
            Sb => 0x3,
            Lk => 0x4,
            Sk => 0x5,

            R => 0x6,
            W => 0x7,
            R2 => 0x8,
            W2 => 0x9,

            Jeq => 0xA,
            Jeq2 => 0xB,
            Jmp => 0xC,
            Jmp2 => 0xD,

            Add => 0xE,
            Xor => 0xF,
        }
    }
}
impl TryFrom<u8> for Instruction {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, ()> {
        use Instruction::*;
        match value {
            0x0 => Ok(La),
            0x1 => Ok(Sa),
            0x2 => Ok(Lb),
            0x3 => Ok(Sb),
            0x4 => Ok(Lk),
            0x5 => Ok(Sk),

            0x6 => Ok(R),
            0x7 => Ok(W),
            0x8 => Ok(R2),
            0x9 => Ok(W2),

            0xA => Ok(Jeq),
            0xB => Ok(Jeq2),
            0xC => Ok(Jmp),
            0xD => Ok(Jmp2),

            0xE => Ok(Add),
            0xF => Ok(Xor),
            _ => Err(()),
        }
    }
}
