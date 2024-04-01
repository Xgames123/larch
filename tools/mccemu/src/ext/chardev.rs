use std::io::Write;

use console::Term;
use libmcc::u4;

use crate::emulator::Emulator;

use super::Extension;
pub struct CharDev {
    current_char: u8,
    stdout: Term,
}

impl CharDev {
    pub fn new() -> Self {
        Self {
            current_char: 0,
            stdout: Term::stdout(),
        }
    }
}
impl Extension for CharDev {
    fn on_mem_write(&mut self, addr: u8, _value: u4, emulator: &Emulator) {
        if addr == 0xF1 {
            let buf = [emulator.ghost_read_mem8(0xF0)];
            self.stdout.write(&buf).unwrap();
        }
    }
    fn on_mem_read(&mut self, addr: u8, _emulator: &Emulator) -> Option<u4> {
        if addr == 0xF0 {
            self.current_char = self.stdout.read_char().unwrap() as u8;
            return Some(u4::from_low(self.current_char));
        };
        if addr == 0xF1 {
            return Some(u4::from_high(self.current_char));
        };
        None
    }
}
