use libmcc::Bank;
use std::collections::HashMap;

use super::{
    lexing::{LexToken, TokenLineNumPair},
    AsmError,
};

fn pad_til_len<T>(vec: &mut Vec<T>, len: usize, val: T)
where
    T: Clone,
{
    for _ in vec.len()..len {
        vec.push(val.clone());
    }
}

pub fn gencode(mut input: Vec<TokenLineNumPair>) -> Result<[Bank; 16], AsmError> {
    let mut out = [Bank::default(); 16];
    let mut data = Vec::new();
    let mut current_bank: u8 = 0;

    let mut labels = HashMap::new();
    let mut label_refs: Vec<(Box<str>, usize)> = Vec::new();

    macro_rules! push {
        ($val:expr) => {
            data.push($val);
        };
    }

    macro_rules! push_bank {
        ($linenum:expr) => {
            for label_ref in label_refs.iter() {
                let addr = *labels.get(&label_ref.0).unwrap_or(&0);
                data[label_ref.1] = addr;
            }
            if out.len() > 16 {
                return Err(AsmError {
                    stage: super::Stage::CodeGen,
                    linenum: $linenum,
                    code_snip: "".into(),
                    message: format!("Bank {} overflow", current_bank).into(),
                });
            }
            pad_til_len(&mut data, 16, 0);
            let bank = TryInto::<[u8; 16]>::try_into(data.clone()).unwrap().into();
            out[current_bank as usize] = bank;
            data.clear();
        };
    }

    let mut last_linenum = None;
    for token in input.drain(..) {
        let linenum = token.linenum;
        let token = token.token;
        match token {
            LexToken::Bank(num) => {
                push_bank!(linenum);
                current_bank = num;
            }
            LexToken::Instruction(inst) => {
                push!(inst.into());
            }
            LexToken::HexLiteral(val) => {
                push!(val);
            }
            LexToken::LabelRef(name) => {
                label_refs.push((name, data.len()));
                push!(0x0);
            }
            LexToken::LabelDef(name) => {
                labels.insert(name, data.len() as u8);
            }
        }
        last_linenum = Some(linenum);
    }

    push_bank!(last_linenum.unwrap_or(1));

    Ok(out)
}
