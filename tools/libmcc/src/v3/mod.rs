use bobbin_bits::U4;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    Nop,
    Push,
    Pop,
    Swp,
    Dswp,
    Di,
    Dd,
    Call,
    Jnz,
    Pushi,
    Popi,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
}

impl Instruction {
    pub fn into_u4(self) -> U4 {
        unsafe { U4::from_u8_unchecked(self as u8) }
    }
    pub fn from_u4(val: U4) -> Self {
        unsafe { std::mem::transmute(val) }
    }
    pub fn try_from_str(string: &str) -> Option<Self> {
        use Instruction::*;
        match string {
            "nop" => Some(Nop),
            "push" => Some(Push),
            "pop" => Some(Pop),
            "swp" => Some(Swp),
            "dswp" => Some(Dswp),
            "di" => Some(Di),
            "dd" => Some(Dd),
            "call" => Some(Call),
            "jnz" => Some(Jnz),
            "pushi" => Some(Pushi),
            "popi" => Some(Popi),

            "inc" => Some(Inc),
            "dec" => Some(Dec),
            "add" => Some(Add),
            "sub" => Some(Sub),
            "mul" => Some(Mul),
            _ => None,
        }
    }
}
