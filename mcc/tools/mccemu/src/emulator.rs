use libmcc::{u4, v3::Instruction};

pub const IP_ADDR0: u8 = 0x00;
pub const IP_ADDR1: u8 = 0x01;
pub const DP_ADDR0: u8 = 0x02;
pub const DP_ADDR1: u8 = 0x03;
pub const SP_ADDR: u8 = 0x04;

pub const STACK_START: u8 = 0x10;

pub struct Emulator {
    pub mem: [u4; 256],
    pub is_running: bool,
    on_mem_acces: Box<dyn Fn(u8, u4, bool, &Self) -> Option<u4>>,
}

impl Emulator {
    pub fn new(
        mem: [u4; 256],
        on_mem_acces: impl Fn(u8, u4, bool, &Self) -> Option<u4> + 'static,
    ) -> Self {
        Emulator {
            mem,
            is_running: false,
            on_mem_acces: Box::new(on_mem_acces),
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

    pub fn sp(&self) -> u4 {
        self.read_mem(SP_ADDR)
    }
    pub fn set_sp(&mut self, value: u4) {
        self.write_mem(SP_ADDR, value)
    }

    pub fn stack_push(&mut self, value: u4) {
        //println!("{:#04x}", value.into_u8());
        self.set_sp((self.sp().overflowing_add(u4::ONE)).into());
        self.write_mem(STACK_START + self.sp().into_low(), value);
    }
    pub fn stack_pop(&mut self) -> u4 {
        let value = self.stack_peek();
        if self.sp() == u4::ZERO {
            return value;
        }
        self.set_sp((self.sp().overflowing_sub(u4::ONE)).into());
        value
    }
    pub fn stack_peek(&mut self) -> u4 {
        let value = self.mem[0x10 as usize + self.sp().into_usize()];
        value
    }

    pub fn start(&mut self) {
        self.set_ip(0x30);
        self.set_dp(0x20);
        self.set_sp(u4::ZERO);
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn read_mem8(&self, addr: u8) -> u8 {
        let lower = self.read_mem(addr).into_low();
        let upper = self.read_mem(addr + 1).into_high();
        return lower | upper;
    }
    ///Read memory without triggering an on_mem_acces call
    pub fn ghost_read_mem8(&self, addr: u8) -> u8 {
        let lower = self.ghost_read_mem(addr).into_low();
        let upper = self.ghost_read_mem(addr + 1).into_high();
        return lower | upper;
    }
    pub fn read_mem(&self, addr: u8) -> u4 {
        if let Some(ext_out) = (self.on_mem_acces)(addr, u4::ZERO, false, self) {
            return ext_out;
        }
        self.ghost_read_mem(addr)
    }
    ///Read memory without triggering an on_mem_acces call
    pub fn ghost_read_mem(&self, addr: u8) -> u4 {
        self.mem[addr as usize]
    }
    pub fn write_mem(&mut self, addr: u8, value: u4) {
        self.mem[addr as usize] = value;
        (self.on_mem_acces)(addr, value, true, self);
    }
    pub fn write_mem8(&mut self, addr: u8, value: u8) {
        self.write_mem(addr, u4::from_low(value));
        self.write_mem(addr + 1, u4::from_high(value));
    }

    pub fn tick(&mut self) -> Option<Instruction> {
        if !self.is_running {
            return None;
        }
        let current_nib = self.read_mem(self.ip());
        let instruct: Instruction = Instruction::from_u4(current_nib);

        let mut jump = false;
        use Instruction::*;
        match instruct {
            Nop => {}
            Psi => {
                //println!("{:#04x}", self.read_mem(0x20));
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
                self.set_dp(self.dp() - 1);
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
                //println!("mdp{}", val);
                let val = self.stack_pop();
                self.write_mem(DP_ADDR1, val);
                //println!("mdp{}", val);
            }
            Di => self.set_dp(self.dp() + 1),
            Dd => self.set_dp(self.dp() - 1),
            Jmp => {
                let val = self.stack_pop();
                self.write_mem(IP_ADDR0, val);
                let val = self.stack_pop();
                self.write_mem(IP_ADDR1, val);
                jump = true;
            }
            Jnz => {
                let val = self.stack_peek();
                if val != u4::ZERO {
                    self.write_mem(IP_ADDR0, self.read_mem(self.dp()));
                    jump = true;
                }
            }
            Inc => {
                let val = self.stack_pop();
                self.stack_push(val.overflowing_add(u4::ONE));
            }
            Dec => {
                let val = self.stack_pop();
                self.stack_push(val.overflowing_sub(u4::ONE));
            }
            Add => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                //println!("add {} {}", a, b);
                self.stack_push(a.overflowing_add(b));
            }
            Sub => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                self.stack_push(a.overflowing_sub(b));
            }
            Mul => {
                let a = self.stack_pop();
                let b = self.stack_pop();
                self.stack_push(a.overflowing_mul(b));
            }
        }
        if self.ip() == 255 {
            self.stop();
            return Some(instruct);
        }
        if !jump {
            self.set_ip(self.ip() + 1);
        }
        Some(instruct)
    }
}
