use crate::util;
use libmcc::Bank;

pub fn emit_hex(data: [Bank; 16], strip: bool) -> Vec<u8> {
    let mut output = String::new();

    for (i, bank) in data.iter().enumerate() {
        if !strip {
            output.push_str("# BANK ");
            output.push_str(&i.to_string());
            output.push_str("\n");
        }
        for nib in bank.data.iter() {
            output.push_str(&util::to_hex4(*nib));
            output.push('\n');
        }
    }

    output.as_bytes().to_vec()
}

pub fn emit_bin_unpacked(data: [Bank; 16]) -> Vec<u8> {
    let mut output = Vec::with_capacity(16 * 16);
    for bank in data.iter() {
        for nib in bank.data.iter() {
            output.push(*nib);
        }
    }
    output
}
pub fn emit_bin_packed(data: [Bank; 16]) -> Vec<u8> {
    let mut output = Vec::with_capacity(16 * 16);
    for bank in data.iter() {
        for nibpair in bank.data.chunks(2).into_iter() {
            let nib1 = nibpair[0];
            let nib2 = nibpair[1];

            output.push(nib1 | nib2 >> 4);
        }
    }
    output
}
