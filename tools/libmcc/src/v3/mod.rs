use crate::u4;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instruction {
    Nop,
    Psi,
    Psd,
    Poi,
    Pod,
    Swp,
    Mdp,
    Di,
    Dd,
    Jmp,
    Jnz,
    Inc,
    Dec,
    Add,
    Sub,
    Mul,
}

impl Instruction {
    pub fn into_u4(self) -> u4 {
        u4::from_low(self as u8)
    }
    pub fn from_u4(val: u4) -> Self {
        unsafe { std::mem::transmute(val) }
    }
    pub fn try_from_str(string: &str) -> Option<Self> {
        use Instruction::*;
        match string {
            "nop" => Some(Nop),
            "psi" => Some(Psi),
            "psd" => Some(Psd),
            "poi" => Some(Poi),
            "pod" => Some(Pod),
            "swp" => Some(Swp),
            "mdp" => Some(Mdp),
            "di" => Some(Di),
            "dd" => Some(Dd),
            "jmp" => Some(Jmp),
            "jnz" => Some(Jnz),

            "inc" => Some(Inc),
            "dec" => Some(Dec),
            "add" => Some(Add),
            "sub" => Some(Sub),
            "mul" => Some(Mul),
            _ => None,
        }
    }
}
