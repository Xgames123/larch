mod u4;
pub use u4::*;

pub enum Reg{
    R0,
    R1,
    R2,
    R3
}

pub enum Instruction{
    Nop,
    Brk,
    Flf,
    Clf,
    Shl,
    Shr,
    //unassigned
    Lim,
    New,
    Mer,
    Mov,
    Jms,
    Jmp,
    Xor,
    Add,
    Mul,
    Cmp,
    Gt,
    Invalid,

}
///Type of the 2 arguments of an operation
enum ArgsTy{
    Split(ArgTy)
    Immediate,
}
///Argument type of an operations
enum ArgTy{
    Reg,
    Const(u2),
}
impl ArgTy{
    pub const fn zero() -> Self{
        Self::Const(u2::ZERO)
    }
}

impl Instruction{
    ///they type of arguments the operation wants
    pub fn arg_types(&self) -> ArgsTy{
        use Self::*;
        match self{
            Nop => ArgsTy::Split(Arg::zero(), Arg::zero()),
            Brk => ArgsTy::Split(Arg::zero(), Arg::Const(u2::ONE)),
            Flf => ArgsTy::Split(Arg::zero(), Arg::Const(u2::TOW)),
            Clf => ArgsTy::Split(Arg::zero(), Arg::Const(u2::THREE)),
            Shl => ArgsTy::Split(Arg::Const(u2::ONE), Arg::Reg),
            Shr => ArgsTy::Split(Arg::Const(u2::TOW), Arg::Reg),

            Lim => ArgsTy::Immediate,
            Mew => ArgsTy::Immediate,
            Mer => ArgsTy::Immediate,
            Mov => ArgsTy::Split(Arg::Reg, Arg::Reg),
            Jms => ArgsTy::Immediate,
            Jmp => ArgsTy::Split(Arg::Reg, Arg::Reg),
            Xor => ArgsTy::Split(Arg::Reg, Arg::Reg),
            Add => 0x08,
            Mul => 0x09,
            Cmp => 0x0A,
            Gt => 0x0B,

            Invalid=>0x00,
        }
    }
    pub fn opcode(&self) -> u4{
        use Self::*;
        u4::from_low(match self{
            Nop => 0x00,
            Brk => 0x00,
            Flf => 0x00,
            Clf => 0x00,
            Shl => 0x00,
            Shr => 0x00,

            Lim => 0x01,
            Mew => 0x02,
            Mer => 0x03,
            Mov => 0x04,
            Jms => 0x05,
            Jmp => 0x06,
            Xor => 0x07,
            Add => 0x08,
            Mul => 0x09,
            Cmp => 0x0A,
            Gt => 0x0B,

            Invalid=>0x00,

        })
    }
    pub fn is_invalid(&self) -> bool{
        match self{
            Invalid => true,
            _=>false,
        }
    }
    pub fn try_from_str(string: &str) -> Self {
        use Self::*;
        match string {
            "nop" => Nop,
            "brk" => Brk,
            "flf" => Flf,
            "clf" => Clf,
            "shl" => Shl,
            "shr" => Shr,

            "lim" => Lim,
            "mew" => Mew,
            "mer" => Mer,
            "mov" => Mov,
            "jms" => Jms,
            "jmp" => Jmp,
            "xor" => Xor,
            "add" => Add,
            "mul" => Mul,
            "cmp" => Cmp,
            "gt" => Gt,

            _ => Invalid,
        }
    }
}
