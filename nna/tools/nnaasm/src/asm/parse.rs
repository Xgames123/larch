use super::AsmError;
use crate::asm::Stage;
use crate::util::{parse_hex4, parse_hex8};
use libnna::Op;
use libnna::{u4, ArgTy};

#[derive(Debug, Clone)]
pub enum Token {
    LabelDef(Box<str>),
    LabelRef { name: Box<str>, wide: bool },
    Org(u8),
    Nib(u4),
}
#[derive(Debug)]
pub struct TokenLineNumPair {
    pub linenum: usize,
    pub token: Token,
}

pub fn parse(input: String) -> Result<Vec<TokenLineNumPair>, AsmError> {
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
                    Token::Org(parse_hex8(&token).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: Stage::Parse,
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
                push_tok!(Token::LabelDef(token[..token.len() - 1].into()), linenum);
                continue;
            }

            if token.starts_with("&&") {
                push_tok!(
                    Token::LabelRef {
                        name: token[2..].into(),
                        wide: true
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("&") {
                push_tok!(
                    Token::LabelRef {
                        name: token[1..].into(),
                        wide: false
                    },
                    linenum
                );
                continue;
            }
            if token.starts_with("0x") {
                push_tok!(
                    Token::Nib(parse_hex4(&token[2..]).ok_or_else(|| AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: Stage::Parse,
                    })?),
                    linenum
                );
                continue;
            }

            match Op::try_from_str(&token) {
                Some(op) => {
                    push_tok!(Token::Nib(u4::from_low(op.opcode())), linenum);

                    let arg = op.arg_types().0;
                    match arg {
                        ArgTy::Any(),
                        ArgTy::Reg(),
                        ArgTy::None(),
                    }
                }
                None => {
                    return Err(AsmError {
                        linenum: Some(linenum),
                        code_snip: token.into(),
                        message: "Invalid operation".into(),
                        stage: Stage::Parse,
                    })
                }
            }
        }
    }

    Ok(vec)
}
