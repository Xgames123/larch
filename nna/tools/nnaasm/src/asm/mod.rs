use std::{ops::Range, rc::Rc};

mod parselex;

const COLOR_RED: &'static str = "\x1b[31m";
const BOLD: &'static str = "\x1b[1m";
const RESET: &'static str = "\x1b[0m";

pub struct AsmError<'a> {
    pub filename: Rc<str>,
    pub file: &'a str,
    pub location: (usize, Range<usize>),
    pub message: String,
}
impl<'a> AsmError<'a> {
    const VIEW_SIZE: usize = 2;

    fn write_gutter(out: &mut String, line_num: Option<usize>, max_len: usize) {
        match line_num {
            Some(lnum) => {
                let lnum = lnum.saturating_add(1);
                for _ in Self::calc_len(lnum)..max_len {
                    out.push(' ');
                }
                out.push_str(&lnum.to_string());
            }
            None => {
                for _ in 0..max_len {
                    out.push(' ');
                }
            }
        };

        out.push_str(" | ");
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
}

impl AsmError<'_> {
    pub fn print(&self) {
        let (linenum, span) = &self.location;
        let max_len = Self::calc_len(linenum.saturating_add(Self::VIEW_SIZE + 1));

        let mut out = String::new();
        for (i, line) in self.file.lines().enumerate() {
            if i < linenum.saturating_sub(Self::VIEW_SIZE)
                || i > linenum.saturating_add(Self::VIEW_SIZE)
            {
                continue;
            }
            Self::write_gutter(&mut out, Some(i), max_len);
            out.push_str(line);
            out.push('\n');
            if i == *linenum {
                Self::write_gutter(&mut out, None, max_len);
                for _ in 0..span.start {
                    out.push(' ');
                }
                out.push_str(COLOR_RED);
                for _ in span.start..span.end {
                    out.push('^');
                }
                out.push_str(RESET);
                out.push(' ');
                out.push_str(&self.message);
                out.push('\n');
            }
        }
        eprintln!(
            "{COLOR_RED}{BOLD}error:{RESET} {}:{}{}\n",
            self.filename, linenum, span.start
        )
    }
}

// pub fn assemble(input: &str) -> Result<[u4; 256], AsmError> {
//     let parsed = parse::parse(input)?;
//     let code = v3::codegen::gencode(lexed)?;
//     Ok(code)
// }
