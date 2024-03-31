use std::cell::{Cell, RefCell};

use clap::ValueEnum;
use libmcc::u4;

use crate::emulator::Emulator;

use self::chardev::CharDev;

mod chardev;

pub trait Extension {
    fn on_mem_read(&mut self, _addr: u8, _emulator: &Emulator) -> Option<u4> {
        None
    }
    fn on_mem_write(&mut self, _addr: u8, _value: u4, _emulator: &Emulator) {}
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ExtType {
    ChardevAscii,
}

macro_rules! type_to_ext {
    ($exts:ident, $($type:ident=>$ext:expr),*) => {{
        let mut extensions = Vec::with_capacity($exts.len());
        for ext in $exts {
            match ext {
                $(
                    ExtType::$type => {
                        let ext: Box<dyn Extension> = Box::new($ext);
                        extensions.push(ext);
                    }
                ),*
            }
        }
        extensions
    }};
}

pub struct ExtManager {
    extensions: RefCell<Vec<Box<dyn Extension>>>,
    ext_vec_borrow: Cell<bool>,
}
impl ExtManager {
    pub fn new(ext_types: Vec<ExtType>) -> Self {
        Self {
            ext_vec_borrow: false.into(),
            extensions: type_to_ext!(ext_types,
                ChardevAscii=>CharDev::new()
            )
            .into(),
        }
    }

    pub fn on_mem_write(&self, addr: u8, value: u4, emulator: &Emulator) {
        if self.ext_vec_borrow.get() {
            return;
        }
        self.ext_vec_borrow.set(true);
        for ext in self.extensions.borrow_mut().iter_mut() {
            ext.on_mem_write(addr, value, emulator);
        }
        self.ext_vec_borrow.set(false);
    }

    pub fn on_mem_read(&self, addr: u8, emulator: &Emulator) -> Option<u4> {
        if self.ext_vec_borrow.get() {
            return None;
        }
        self.ext_vec_borrow.set(true);
        for ext in self.extensions.borrow_mut().iter_mut() {
            let out = ext.on_mem_read(addr, emulator);
            if out.is_some() {
                self.ext_vec_borrow.set(false);
                return out;
            }
        }
        self.ext_vec_borrow.set(false);
        None
    }
}
