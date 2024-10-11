mod usmol;
pub use usmol::*;

pub enum Reg {
    R0,
    R1,
    R2,
    R3,
}

///Argument type of an operation
pub enum ArgTy {
    None(),
    Any(&'static str),
    Reg(&'static str),
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
            pub fn arg_types(&self) -> ArgTy{
                match self.0{
                    $($opcode => oparg!($arg0)),*,
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
    pub NnaOp{
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
        "xor":0x70,("source":reg),   ("a":reg),
        "add":0x80,("source":reg),   ("a":reg),
        "mul":0x90,("source":reg),   ("a":reg),
        "cmp":0xA0,("a":reg),        ("b":reg),
        "gt" :0xB0,("a":reg),        ("b":reg)
    }
}
