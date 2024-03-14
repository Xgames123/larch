use libmcc::bobbin_bits::U4;
use std::collections::HashMap;

use crate::asm::Stage;

use super::{
    super::AsmError,
    lexing::{LexToken, TokenLineNumPair},
};

fn pad_til_len<T>(vec: &mut Vec<T>, len: usize, val: T)
where
    T: Clone,
{
    for _ in vec.len()..len {
        vec.push(val.clone());
    }
}

struct LabelRef {
    name: Box<str>,
    addr: u8,
    wide: bool,
    linenum: usize,
}

pub fn gencode(mut input: Vec<TokenLineNumPair>) -> Result<[U4; 256], AsmError> {
    let mut output = [U4::B0000; 256];
    let mut data = Vec::new();
    let mut current_org: u8 = 0;

    let mut orgs = vec![];
    let mut labels = HashMap::new();
    let mut label_refs: Vec<LabelRef> = Vec::new();

    for token in input.drain(..) {
        let linenum = token.linenum;
        let token = token.token;
        match token {
            LexToken::Org(num) => {
                for (i, nib) in data.iter().enumerate() {
                    output[current_org as usize + i] = *nib;
                    //println!("{} = {:#x}", current_org as usize + i, nib.into_u8());
                }
                //println!("org end {}", current_org);
                current_org = num;
                orgs.push(num);
                data.clear();
            }
            LexToken::Instruction(inst) => {
                data.push(inst.into_u4());
            }
            LexToken::HexLiteral(val) => {
                data.push(val.into());
            }
            LexToken::LabelRef { name, wide } => {
                label_refs.push(LabelRef {
                    name,
                    addr: current_org + data.len() as u8,
                    wide,
                    linenum,
                });
                data.push(U4::B0000);
                if wide {
                    data.push(U4::B0000);
                }
            }
            LexToken::LabelDef(name) => {
                labels.insert(name, current_org + data.len() as u8);
            }
        }
    }
    //write last org
    for (i, nib) in data.iter().enumerate() {
        output[current_org as usize + i] = *nib;
        //println!("{} = {:#x}", current_org as usize + i, nib.into_u8());
    }

    //resolve labels
    for LabelRef {
        name,
        addr,
        wide,
        linenum,
    } in label_refs
    {
        let label_addr = labels.get(&name).ok_or_else(|| AsmError {
            linenum,
            message: "label not defined".into(),
            code_snip: name,
            stage: Stage::CodeGen,
        })?;
        output[addr as usize + 1] = (label_addr & 0x0F).into();
        if wide {
            output[addr as usize] = (label_addr >> 4 & 0x0F).into();
        }
    }

    Ok(output)
}
