use libmcc::{bobbin_bits::U4, v3::Instruction};

pub const IP_ADDR0: u8 = 0x00;
pub const IP_ADDR1: u8 = 0x01;
pub const DP_ADDR0: u8 = 0x02;
pub const DP_ADDR1: u8 = 0x03;
pub const SP_ADDR: u8 = 0x04;

pub const STACK_START: u8 = 0x10;

pub struct Emulator {
    pub mem: [U4; 256],
    pub is_running: bool,
}

impl Emulator {
    pub fn new(mem: [U4; 256]) -> Self {
        Emulator {
            mem,
            is_running: false,
        }
    }
    pub fn set_dp(&mut self, value: u8) {
        self.write_mem8(DP_ADDR0, value);
    }
    pub fn dp(&self) -> u8 {
        self.read_mem8(DP_ADDR0)
    }

    pub fn set_ip(&mut self, value: u8) {
        self.write_mem8(IP_ADDR0, value)
    }
    pub fn ip(&self) -> u8 {
        self.read_mem8(IP_ADDR0)
    }

    pub fn sp(&self) -> U4 {
        self.read_mem(SP_ADDR)
    }
    pub fn set_sp(&mut self, value: U4) {
        self.write_mem(SP_ADDR, value)
    }

    pub fn stack_push(&mut self, value: U4) {
        self.set_sp((self.sp().into_u8() + 1).into());
        self.write_mem(STACK_START + self.sp().into_u8(), value);
    }
    pub fn stack_pop(&mut self) -> U4 {
        let value = self.stack_peek();
        if self.sp() == 0 {
            return value;
        }
        self.set_sp((self.sp().into_u8() - 1).into());
        value
    }
    pub fn stack_peek(&mut self) -> U4 {
        let value = self.mem[0x10 as usize + self.sp().into_usize()];
        value
    }

    pub fn start(&mut self) {
        self.set_ip(0x30);
        self.set_dp(0x20);
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn read_mem8(&self, addr: u8) -> u8 {
        let lower = self.read_mem(addr).into_u8();
        let upper = self.read_mem(addr + 1).into_u8() << 4;
        return lower | upper;
    }
    pub fn read_mem(&self, addr: u8) -> U4 {
        self.mem[addr as usize]
    }
    pub fn write_mem(&mut self, addr: u8, value: U4) {
        self.mem[addr as usize] = value;
    }
    pub fn write_mem8(&mut self, addr: u8, value: u8) {
        let lower = value & 0x0F;
        let upper = value >> 4 & 0x0F;
        self.write_mem(addr, lower.into());
        self.write_mem(addr + 1, upper.into());
    }

    pub fn tick(&mut self) -> Option<Instruction> {
        if !self.is_running {
            return None;
        }
        let current_nib = self.read_mem(self.ip());
        let instruct: Instruction = Instruction::from_u4(current_nib);
        use Instruction::*;
        match instruct {
            Nop => {}
            Psi => {
                //println!("{:#04x}", self.dp());
                self.stack_push(self.read_mem(self.dp()));
                self.set_dp(self.dp() + 1);
            }
            Psd => {
                //println!("{:#04x}", self.dp());
                self.stack_push(self.read_mem(self.dp()));
                self.set_dp(self.dp() - 1);
            }
            Poi => {
                let val = self.stack_pop();
                self.write_mem(self.dp(), val);
                self.set_dp(self.dp() + 1);
            }
            Pod => {
                let val = self.stack_pop();
                self.write_mem(self.dp(), val);
                self.set_dp(self.dp() + 1);
            }
            Swp => {
                let val1 = self.stack_pop();
                let val2 = self.stack_pop();
                self.stack_push(val1);
                self.stack_push(val2);
            }
            Mdp => {
                let val = self.stack_pop();
                self.write_mem(DP_ADDR0, val);
                let val = self.stack_pop();
                self.write_mem(DP_ADDR1, val);
            }
            Di => self.set_dp(self.dp() + 1),
            Dd => self.set_dp(self.dp() - 1),
            Jmp => {
                let val = self.stack_pop();
                self.write_mem(IP_ADDR0, val);
                let val = self.stack_pop();
                self.write_mem(IP_ADDR1, val);
            }
            Jnz => {
                let val = self.stack_peek();
                if val != 0 {
                    self.write_mem(DP_ADDR0, self.read_mem(self.dp()))
                }
            }
            Inc => {
                let val = self.stack_pop();
                self.stack_push((val.into_u8() + 1).into());
            }
            Dec => {
                let val = self.stack_pop();
                self.stack_push((val.into_u8() - 1).into());
            }
            Add => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                self.stack_push((a.into_u8() + b.into_u8()).into());
            }
            Sub => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                self.stack_push((a.into_u8() - b.into_u8()).into());
            }
            Mul => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                self.stack_push((a.into_u8() * b.into_u8()).into());
            }
        }
        if self.ip() == 255 {
            self.stop();
            return Some(instruct);
        }
        self.set_ip(self.ip() + 1);
        Some(instruct)
    }
}
