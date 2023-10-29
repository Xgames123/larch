use std::str::FromStr;

pub fn to_hex4(val: u8) -> String {
    let mut str = String::from_str("0x").unwrap();
    str.push(char::from_digit(val as u32, 16).unwrap());
    str
}

pub fn parse_hex4<'a>(str: &'a str) -> Option<u8> {
    let str = str.to_lowercase();
    if str.len() != 1 {
        return None;
    }
    for char in str.chars() {
        return char.to_digit(16).map(|val| val as u8);
    }
    return None;
}
