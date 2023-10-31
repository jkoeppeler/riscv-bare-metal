#![no_std]

use volatile_register::{RW, RO};

pub struct Uart {
    p: &'static mut uartRegs,
}

#[repr(C)]
struct uartRegs {
    pub rx_tx_reg: RW<u8>,
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            p: unsafe { &mut *(0x1000_0000 as *mut uartRegs) }
        }
    }

    pub fn write(&self, c: u8) {
        unsafe {self.p.rx_tx_reg.write(c)}
    }

    pub fn read(&self) -> u8 {
        return unsafe {self.p.rx_tx_reg.read()}
    }
}
