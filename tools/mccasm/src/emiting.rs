use std::path::Path;

use libmcc::bobbin_bits::U4;

use crate::{util, Format};

pub fn emit_hex(data: [U4; 256]) -> Vec<u8> {
    let mut output = String::new();

    for nib in data.iter() {
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
pub fn emit(format: Format, file_ext: Option<&str>, code: [U4; 256], strip: bool) -> Vec<u8> {
    match format {
        Format::Hex => emit_hex(code),
        Format::Bin => emit_bin_packed(code),
        Format::Ubin => emit_bin_unpacked(code),
        Format::Auto => emit(
            file_ext
                .map(|ext| match ext {
                    "hex" => Some(Format::Hex),
                    "bin" => Some(Format::Bin),
                    _ => None,
                })
                .flatten()
                .unwrap_or(Format::Bin),
            file_ext,
            code,
            strip,
        ),
    }
}
