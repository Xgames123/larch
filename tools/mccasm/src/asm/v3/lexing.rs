use super::super::AsmError;
use crate::util::{parse_hex4, parse_hex8};
use libmcc::bobbin_bits::U4;
use libmcc::v3::Instruction;

#[derive(Debug, Clone)]
pub enum LexToken {
    Instruction(Instruction),
    LabelDef(Box<str>),
    LabelRef { name: Box<str>, wide: bool },
    Org(u8),
    HexLiteral(U4),
}
#[derive(Debug)]
pub struct TokenLineNumPair {
    pub linenum: usize,
    pub token: LexToken,
}

pub fn lex(input: String) -> Result<Vec<TokenLineNumPair>, AsmError> {
    let mut vec = Vec::new();

    macro_rules! push_tok {
        ($token:expr, $linenum:expr) => {
            vec.push(TokenLineNumPair {
                linenum: $linenum,
                token: $token,
            })
        };
    }

    for (linenum, line) in input.split("\n").enumerate() {
        let linenum = linenum + 1;
        let mut line = line;

        // strip comments
        if let Some(pos) = line.find('#') {
            line = &line[..pos];
        }
        if let Some(pos) = line.find(';') {
            line = &line[..pos];
        }

        let mut org = false;
        for token in line.split_whitespace() {
            if org {
                push_tok!(
                    LexToken::Org(parse_hex8(&token).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: super::super::Stage::Lex,
                    })?),
                    linenum
                );
                continue;
            }
            if token == ".org" {
                org = true;
                continue;
            }

            if token.ends_with(":") {
                push_tok!(LexToken::LabelDef(token[..token.len() - 1].into()), linenum);
                continue;
            }

            if token.starts_with("&&") {
                push_tok!(
                    LexToken::LabelRef {
                        name: token[2..].into(),
                        wide: true
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("&") {
                push_tok!(
                    LexToken::LabelRef {
                        name: token[1..].into(),
                        wide: false
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("0x") {
                push_tok!(
                    LexToken::HexLiteral(parse_hex4(&token[2..]).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: super::super::Stage::Lex,
                    })?),
                    linenum
                );
                continue;
            }

            push_tok!(
                LexToken::Instruction(Instruction::try_from_str(&token).ok_or_else(|| {
                    AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Invalid instruction".into(),
                        stage: super::super::Stage::Lex,
                    }
                })?),
                linenum
            );
        }
    }

    Ok(vec)
}
