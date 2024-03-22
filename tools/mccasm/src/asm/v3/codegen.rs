use libmcc::{u4, v3::Instruction};
use log::*;
use std::{collections::HashMap, fmt::Display};

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

#[derive(Clone, Copy)]
struct Org {
    start_addr: u8,
    size: u8,
    linenum: Option<usize>,
}
impl Display for Org {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            ".org {:#04x} ; size: {:#04x}",
            self.start_addr, self.size
        ))
    }
}
impl Org {
    pub fn end_addr(self) -> u8 {
        self.start_addr + self.size
    }

    pub fn overlap(self, other: Org) -> bool {
        if other.start_addr < self.start_addr && other.end_addr() < self.start_addr {
            false
        } else if other.start_addr > self.end_addr() {
            false
        } else {
            true
        }
    }
}

#[inline]
fn write_org(
    mut org: Org,
    data: &Vec<u4>,
    output: &mut [u4; 256],
    orgs: &[Org],
) -> Result<Org, AsmError> {
    let size = data.len();
    org.size = size as u8;
    for other_org in orgs {
        if other_org.start_addr == org.start_addr {
            continue;
        }
        if other_org.overlap(org) {
            return Err(AsmError {
                linenum: org.linenum,
                message: format!("is overlapping '{}'", other_org).into(),
                code_snip: format!("{}", org).into(),
                stage: Stage::CodeGen,
            });
        }
    }

    trace!("org begin {:#x}", org.start_addr);
    for (i, nib) in data.iter().enumerate() {
        output[org.start_addr as usize + i] = *nib;
        trace!(
            "{:#x} = {:#x} {:?}",
            org.start_addr as usize + i,
            nib.into_low(),
            Instruction::from_u4(*nib)
        );
    }
    trace!("org end {:#x} size: {:#04x}", org.start_addr, size);
    Ok(org)
}

pub fn gencode(mut input: Vec<TokenLineNumPair>) -> Result<[u4; 256], AsmError> {
    let mut output = [u4::ZERO; 256];
    let mut data = Vec::new();
    let mut current_org: Org = Org {
        linenum: None,
        start_addr: 0,
        size: 0,
    };

    let mut labels = HashMap::new();
    let mut label_refs: Vec<LabelRef> = Vec::new();
    let mut orgs = Vec::new();

    for token in input.drain(..) {
        let linenum = token.linenum;
        let token = token.token;
        match token {
            LexToken::Org(start_addr) => {
                let org = write_org(current_org, &data, &mut output, &orgs)?;
                current_org = Org {
                    start_addr,
                    size: 0,
                    linenum: Some(linenum),
                };
                orgs.push(org);
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
                    addr: current_org.start_addr + data.len() as u8,
                    wide,
                    linenum,
                });
                data.push(u4::ZERO);
                if wide {
                    data.push(u4::ZERO);
                }
            }
            LexToken::LabelDef(name) => {
                let addr = current_org.start_addr + data.len() as u8;
                trace!(
                    "{}: addr: {:#04x} data: {:#04x} {:?}   org: {:#04x}, data_len: {:#04x}",
                    name,
                    addr,
                    output[addr as usize].into_low(),
                    Instruction::from_u4(output[addr as usize]),
                    current_org.start_addr,
                    data.len()
                );
                labels.insert(name, addr);
            }
        }
    }
    //write last org
    write_org(current_org, &data, &mut output, &orgs)?;

    resolve_labels(&mut output, labels, label_refs)?;

    Ok(output)
}

fn resolve_labels(
    output: &mut [u4; 256],
    labels: HashMap<Box<str>, u8>,
    label_refs: Vec<LabelRef>,
) -> Result<(), AsmError> {
    trace!("resolving labels");
    for LabelRef {
        name,
        addr,
        wide,
        linenum,
    } in label_refs
    {
        if wide {
            trace!("&&{}", name);
        } else {
            trace!("&{}", name);
        }
        let label_addr = labels.get(&name).ok_or_else(|| AsmError {
            linenum: Some(linenum),
            message: "label not defined".into(),
            code_snip: name,
            stage: Stage::CodeGen,
        })?;
        trace!(
            "{:#x} = {:#x}",
            (addr as usize) as u8,
            (label_addr >> 4 & 0x0F)
        );
        if wide {
            trace!(
                "{:#x} = {:#x}",
                (addr as usize + 1) as u8,
                (label_addr & 0x0F)
            );
        }

        trace!(
            "label pointing to {:#x} {:?}",
            output[*label_addr as usize].into_low(),
            Instruction::from_u4(output[*label_addr as usize])
        );
        if wide {
            output[addr as usize + 1] = u4::from_low(*label_addr);
            output[addr as usize] = u4::from_high(*label_addr);
        } else {
            output[addr as usize] = u4::from_low(*label_addr);
        }
    }

    Ok(())
}
