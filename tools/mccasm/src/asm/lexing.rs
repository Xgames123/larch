use super::AsmError;
use crate::util::parse_hex4;
use libmcc::Instruction;

#[derive(Debug, Clone)]
pub enum LexToken {
    Instruction(libmcc::Instruction),
    LabelDef(Box<str>),
    LabelRef(Box<str>),
    Bank(u8),
    HexLiteral(u8),
}
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

        if let Some(pos) = line.find('#') {
            line = &line[..pos];
        }

        let mut bank = false;
        for token in line.split_whitespace() {
            if bank {
                push_tok!(
                    LexToken::Bank(parse_hex4(&token).ok_or_else(|| AsmError {
                        linenum,
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: super::Stage::Lex,
                    })?),
                    linenum
                );
                continue;
            }
            if token == ".bank" {
                bank = true;
                continue;
            }

            if token.ends_with(":") {
                push_tok!(LexToken::LabelDef(token[..token.len() - 1].into()), linenum);
                continue;
            }

            if token.starts_with("&") {
                push_tok!(LexToken::LabelRef(token[1..].into()), linenum);
                continue;
            }
            if token.starts_with("0x") {
                push_tok!(
                    LexToken::HexLiteral(parse_hex4(&token[2..]).ok_or_else(|| AsmError {
                        linenum,
                        code_snip: token.into(),
                        message: "Failed to parse hex digit".into(),
                        stage: super::Stage::Lex,
                    })?),
                    linenum
                );
                continue;
            }

            push_tok!(
                LexToken::Instruction(lex_instruct(&token).ok_or_else(|| AsmError {
                    linenum,
                    code_snip: token.into(),
                    message: "Invalid instruction".into(),
                    stage: super::Stage::Lex,
                })?),
                linenum
            );
        }
    }

    Ok(vec)
}

fn lex_instruct<'a>(token: &'a str) -> Option<Instruction> {
    use libmcc::Instruction::*;
    match token {
        "la" => Some(La),
        "sa" => Some(Sa),
        "lb" => Some(Lb),
        "sb" => Some(Sb),
        "lk" => Some(Lk),
        "sk" => Some(Sk),

        "r" => Some(R),
        "w" => Some(W),
        "r2" => Some(R2),
        "w2" => Some(W2),

        "jeq" => Some(Jeq),
        "jeq2" => Some(Jeq2),
        "jmp" => Some(Jmp),
        "jmp2" => Some(Jmp2),

        "add" => Some(Add),
        "xor" => Some(Xor),
        _ => None,
    }
}
