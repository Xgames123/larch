use std::fmt::Display;

use libmcc::Bank;

mod codegen;
mod lexing;

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

pub fn assemble(input: String) -> Result<[Bank; 16], AsmError> {
    let lexed = lexing::lex(input)?;
    Ok(codegen::gencode(lexed)?)
}
