use libmcc::bobbin_bits::U4;

use crate::util;

pub fn emit_hex(data: [U4; 256], strip: bool) -> Vec<u8> {
    let mut output = String::new();

    for (i, nib) in data.iter().enumerate() {
        //println!("{} {:#x}", util::to_hex4(*nib), nib.into_u8());
        output.push_str(&util::to_hex4(*nib));
        output.push('\n');
    }

    output.as_bytes().to_vec()
}

pub fn emit_bin_unpacked(data: [U4; 256]) -> Vec<u8> {
    let mut output = Vec::with_capacity(16 * 16);
    for nib in data.iter() {
        output.push(nib.into_u8());
    }
    output
}
pub fn emit_bin_packed(data: [U4; 256]) -> Vec<u8> {
    let mut output = Vec::with_capacity(16 * 16);
    for nibpair in data.chunks(2).into_iter() {
        let nib0 = nibpair[0].into_u8();
        let nib1 = nibpair[1].into_u8();

        output.push(nib0 << 4 | nib1);
    }
    output
}
