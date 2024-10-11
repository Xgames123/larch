use libnna::u4;
use std::fmt::Display;

mod parse;

pub enum Stage {
    Parse,
    Asm,
}
impl Display for Stage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stage::Parse => fmt.write_str("PARSE"),
            Stage::Asm => fmt.write_str("ASM"),
        }
    }
}
pub struct AsmError {
    linenum: Option<usize>,
    code_snip: Box<str>,
    message: Box<str>,
    stage: Stage,
}
impl Display for AsmError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(linenum) = self.linenum {
            fmt.write_fmt(format_args!(
                "{} line {} '{}' {}",
                self.stage, linenum, self.code_snip, self.message
            ))
        } else {
            fmt.write_fmt(format_args!(
                "{} '{}' {}",
                self.stage, self.code_snip, self.message
            ))
        }
    }
}

pub fn assemble(input: String) -> Result<[u4; 256], AsmError> {
    let parsed = parse::parse(input)?;
    let code = v3::codegen::gencode(lexed)?;
    Ok(code)
}
