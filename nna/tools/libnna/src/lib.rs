mod usmol;
use std::fmt::Display;

pub use usmol::*;

pub enum Reg {
    R0,
    R1,
    R2,
    R3,
}
impl Reg {
    pub fn code(&self) -> u2 {
        match self {
            Self::R0 => u2!(0b00),
            Self::R1 => u2!(0b01),
            Self::R2 => u2!(0b10),
            Self::R3 => u2!(0b11),
        }
    }
}

///Argument type of an operation
pub enum ArgTy {
    None(),
    Any(&'static str),
    Reg(&'static str),
}
impl Display for ArgTy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None() => {}
            Self::Any(name) => f.write_str(*name)?,
            Self::Reg(name) => {
                f.write_str("[")?;
                f.write_str(name)?;
                f.write_str("]")?
            }
        }
        Ok(())
    }
}

macro_rules! oparg {
    (($desc:literal:reg)) => {
        ArgTy::Reg($desc)
    };
    (($desc:literal:raw)) => {
        ArgTy::Any($desc)
    };
    ((none)) => {
        ArgTy::None()
    };
}

macro_rules! ops {
    ($vis:vis $name:ident{$($opname:literal:$opcode:literal,$arg0:tt,$arg1:tt),*}) => {
        $vis struct $name(u8);
        impl $name{
            pub fn opcode(&self) -> u8{
                self.0
            }
            pub fn arg_types(&self) -> (ArgTy, ArgTy){
                match self.0{
                    $($opcode => (oparg!($arg0), oparg!($arg1))),*,
                    _=>unreachable!(),
                }

            }
            pub fn try_from_str(string: &str) -> Option<Self>{
                match string{
                    $($opname => Some($name($opcode))),*,
                    _=>None,
                }
            }
        }
    };
}
ops! {
    pub Op{
        "nop":0x00,(none),(none),
        "brk":0x01,(none),(none),
        "flf":0x02,(none),(none),
        "clf":0x03,(none),(none),
        "shl":0x04,(none),("reg":reg),
        "shr":0x08,(none),("reg":reg),

        "lim":0x10,("value_low":raw),("value_high":raw),
        "mew":0x20,("addr_low":raw), ("addr_high":raw),
        "mer":0x30,("addr_low":raw), ("addr_high":raw),
        "mov":0x40,("source":reg),   ("dest":reg),
        "jms":0x50,("addr_low":raw), ("addr_high":raw),
        "jmp":0x60,("addr":reg),     ("bank":reg),
        "eq" :0x70,("a":reg),        ("b":reg),
        "gt" :0x80,("a":reg),        ("b":reg),
        "add":0x90,("source":reg),   ("a":reg),
        "mul":0xA0,("source":reg),   ("a":reg),
        "and":0xB0,("source":reg),   ("a":reg),
        "nand":0xC0,("source":reg),   ("a":reg),
        "or" :0xD0,("source":reg),   ("a":reg),
        "xor":0xE0,("source":reg),   ("a":reg)
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.write_str(j)
        let (a1, a2) = self.arg_types();
    }
}
