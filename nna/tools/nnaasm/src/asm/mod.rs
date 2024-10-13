use libnna::u4;
use std::{
    fmt::{Display, Write},
    ops::Range,
};

//mod parse;

pub struct AsmError<'a> {
    pub filename: Box<str>,
    pub file: &'a str,
    pub linenum: usize,
    pub span: Range<usize>,
    pub message: Box<str>,
}
impl<'a> AsmError<'a> {
    const VIEW_SIZE: usize = 2;
}

fn write_gutter(
    fmt: &mut std::fmt::Formatter<'_>,
    line_num: Option<usize>,
    max_len: usize,
) -> std::fmt::Result {
    match line_num {
        Some(lnum) => {
            let lnum = lnum.saturating_add(1);
            for _ in calc_len(lnum)..max_len {
                fmt.write_str(" ")?;
            }
            lnum.fmt(fmt)?;
        }
        None => {
            for _ in 0..max_len {
                fmt.write_str(" ")?;
            }
        }
    };

    fmt.write_str(" | ")?;
    Ok(())
}
fn calc_len(num: usize) -> usize {
    let mut digits = 0;
    let mut i = 1;
    while i <= num {
        digits += 1;
        i *= 10;
    }
    digits
}
impl<'a> Display for AsmError<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_len = calc_len(self.linenum.saturating_add(Self::VIEW_SIZE + 1));

        fmt.write_str("error: ")?;
        fmt.write_str(&self.filename)?;
        fmt.write_str(":")?;
        self.linenum.fmt(fmt)?;
        fmt.write_str(":")?;
        self.span.start.fmt(fmt)?;
        fmt.write_str("\n")?;
        for (i, line) in self.file.lines().enumerate() {
            if i < self.linenum.saturating_sub(Self::VIEW_SIZE)
                || i > self.linenum.saturating_add(Self::VIEW_SIZE)
            {
                continue;
            }
            write_gutter(fmt, Some(i), max_len)?;
            fmt.write_str(line)?;
            fmt.write_str("\n")?;
            if i == self.linenum {
                write_gutter(fmt, None, max_len)?;
                for _ in 0..self.span.start {
                    fmt.write_char(' ')?;
                }
                for _ in self.span.start..self.span.end {
                    fmt.write_char('^')?;
                }
                fmt.write_str(" ")?;
                fmt.write_str(&self.message)?;
                fmt.write_str("\n")?;
            }
        }
        Ok(())
    }
}

// pub fn assemble(input: &str) -> Result<[u4; 256], AsmError> {
//     let parsed = parse::parse(input)?;
//     let code = v3::codegen::gencode(lexed)?;
//     Ok(code)
// }
