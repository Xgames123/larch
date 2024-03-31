use std::io::Write;

use libmcc::u4;

use crate::emulator::Emulator;

use super::Extension;
pub struct CharDev {}

impl CharDev {
    pub fn new() -> Self {
        Self {}
    }
}
impl Extension for CharDev {
    fn on_mem_write(&mut self, addr: u8, _value: u4, emulator: &Emulator) {
        if addr == 0xF1 {
            let buf = [emulator.read_mem8(0xF0)];
            std::io::stdout().write_all(&buf).unwrap();
            std::io::stdout().flush().unwrap();
        }
    }
}
