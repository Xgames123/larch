use libmcc::bobbin_bits::U4;
use log::*;
use std::collections::HashMap;

use crate::asm::Stage;

use super::{
    super::AsmError,
    lexing::{LexToken, TokenLineNumPair},
};

struct LabelRef {
    name: Box<str>,
    addr: u8,
    wide: bool,
    linenum: usize,
}

#[inline]
fn write_org(org_num: u8, data: &Vec<U4>, output: &mut [U4; 256]) {
    trace!("org begin {:#x}", org_num);
    for (i, nib) in data.iter().enumerate() {
        output[org_num as usize + i] = *nib;
        trace!("{:#x} = {:#x}", org_num as usize + i, nib.into_u8());
    }
    trace!("org end {:#x}", org_num);
}

pub fn gencode(mut input: Vec<TokenLineNumPair>) -> Result<[U4; 256], AsmError> {
    let mut output = [U4::B0000; 256];
    let mut data = Vec::new();
    let mut current_org: u8 = 0;

    let mut labels = HashMap::new();
    let mut label_refs: Vec<LabelRef> = Vec::new();

    for token in input.drain(..) {
        let linenum = token.linenum;
        let token = token.token;
        match token {
            LexToken::Org(num) => {
                write_org(current_org, &data, &mut output);
                current_org = num;
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
    write_org(current_org, &data, &mut output);
    data.clear();

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
