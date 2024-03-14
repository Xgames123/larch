use libmcc::{bobbin_bits::U4, InstructionSet};
use std::fmt::Display;
//pub mod v2;
pub mod v3;

pub enum Stage {
    Lex,
    CodeGen,
}
impl Display for Stage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::Lex => fmt.write_str("LEX"),
            Stage::CodeGen => fmt.write_str("CGEN"),
        }
    }
}
pub struct AsmError {
    linenum: usize,
    code_snip: Box<str>,
    message: Box<str>,
    stage: Stage,
}
impl Display for AsmError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_fmt(format_args!(
            "{} line {} '{}' {}",
            self.stage, self.linenum, self.code_snip, self.message
        ))
    }
}

pub fn assemble(input: String, iset: InstructionSet) -> Result<[U4; 256], AsmError> {
    let lexed = v3::lexing::lex(input)?;
    let code = v3::codegen::gencode(lexed)?;
    //println!("{:?}", code);
    Ok(code)
}
