use libmcc::bobbin_bits::U4;
use std::str::FromStr;

pub fn to_hex4(val: U4) -> String {
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

pub fn parse_hex4<'a>(str: &'a str) -> Option<U4> {
    let str = str.to_lowercase();
    if str.len() != 1 {
        return None;
    }
    for char in str.chars() {
        return char.to_digit(16).map(|val| U4::from(val));
    }
    return None;
}
