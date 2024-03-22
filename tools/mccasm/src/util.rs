use libmcc::u4;
use std::str::FromStr;

pub fn to_hex4(val: u4) -> String {
    let mut str = String::from_str("0x").unwrap();
    str.push(char::from_digit(val.into_u32(), 16).unwrap());
    str
}

pub fn parse_hex8<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 2 {
        return None;
    }

    u8::from_str_radix(&str, 16).ok()
}

pub fn parse_hex4<'a>(str: &'a str) -> Option<u4> {
    let str = str.to_lowercase();
    if str.len() != 1 {
        return None;
    }
    for char in str.chars() {
        return char.to_digit(16).map(|val| u4::from_u32(val));
    }
    return None;
}

pub fn count_nonzero_pages(data: &[u4; 256]) -> usize {
    let mut non_zero_pages = 0;
    for page in data.chunks(16) {
        let mut zero = true;
        for nib in page {
            if *nib != u4::ZERO {
                zero = false;
                break;
            }
        }
        if zero {
            non_zero_pages += 1;
        }
    }
    non_zero_pages
}
