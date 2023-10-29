use libmcc::{Bank, Instruction, Memory};
pub struct Emulator {
    pub mem: Memory,
    pub is_running: bool,

    pub rw: u8,
    pub ra: u8,
    pub rb: u8,
    pub rk: u8,

    pub pc: u8,
}

impl Emulator {
    pub fn new(mem: Memory) -> Self {
        Emulator {
            mem,
            is_running: false,
            rw: 0,
            ra: 0,
            rb: 0,
            rk: 0,
            pc: 0,
        }
    }

    pub fn start(&mut self) {
        self.pc = 0;
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    fn fetch_mem(&self, addr: u8, bank: u8) -> u8 {
        self.mem[bank as usize].data[addr as usize]
    }

    pub fn tick(&mut self) {
        let pc_lower = self.pc & 0xF0;
        let pc_upper = self.pc & 0x0F << 4;
        let current_nib = self.fetch_mem(pc_lower, pc_upper);

        let instruct: Instruction = current_nib.try_into().unwrap();
    }
}
