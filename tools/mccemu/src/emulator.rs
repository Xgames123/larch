use libmcc::{bobbin_bits::U4, v3::Instruction};

const IP_ADDR0: u8 = 0x00;
const IP_ADDR1: u8 = 0x01;
const DP_ADDR0: u8 = 0x02;
const DP_ADDR1: u8 = 0x03;
const SP_ADDR: u8 = 0x04;

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
        self.write_mem(0x10 + self.sp().into_u8(), value);
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

    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }
        let current_nib = self.read_mem(self.ip());
        let instruct: Instruction = Instruction::from_u4(current_nib);
        use Instruction::*;
        match instruct {
            Nop => {}
            Push => {
                //println!("{:#04x}", self.dp());
                self.stack_push(self.read_mem(self.dp()))
            }
            Pop => {
                let val = self.stack_pop();
                self.write_mem(self.dp(), val);
            }
            Swp => {
                let val1 = self.stack_pop();
                let val2 = self.stack_pop();
                self.stack_push(val1);
                self.stack_push(val2);
            }
            Dswp => {
                let dp0 = self.read_mem(DP_ADDR0);
                let dp1 = self.read_mem(DP_ADDR1);
                let new_dp0 = self.stack_pop();
                let new_dp1 = self.stack_pop();
                self.write_mem(DP_ADDR0, new_dp0);
                self.write_mem(DP_ADDR1, new_dp1);
                self.stack_push(dp1);
                self.stack_push(dp0);
            }
            Di => self.set_dp(self.dp() + 1),
            Dd => self.set_dp(self.dp() - 1),
            Call => {}
            Jnz => {
                let val = self.stack_peek();
                if val != 0 {
                    self.write_mem(DP_ADDR0, self.read_mem(self.dp()))
                }
            }
            Pushi => {}
            Popi => {}
            Inc => {}
            Dec => {}
            Add => {}
            Sub => {}
            Mul => {}
        }
        if self.ip() == 255 {
            self.stop();
            return;
        }
        self.set_ip(self.ip() + 1);
    }
}
