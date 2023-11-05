use volatile_register::{RW, RO, WO};
use my_riscv_macros::*;

#[derive(Clone, Copy)]
enum IER {
    DataRdy = 0,
    ThrEmpty = 1, 
}

pub struct Uart {
    p: &'static mut uartRegs,
}

#[repr(C)]
#[derive(GetFn)]
struct uartRegs {
    pub rhr_thr: RW<u8>,    // Receiver/Transmitter Holding Register
    pub ier: RW<u8>,        //Interrupt Enable Register
    pub isr_fcr: RW<u8>,    // Interrupt Status Register (R), Fifo Control Register (W)
    pub lcr: RW<u8>,        // Line Control Register
    pub mcr: RW<u8>,        // Modem Control Register
    pub lsr: RO<u8>,        // Line Status Register
    pub msr: RO<u8>,        // Modem Status Register
    pub spr: RW<u8>,        // Scratch Pad Register 
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            p: unsafe { &mut *(0x1000_0000 as *mut uartRegs) }
        }
    }

    pub fn write(&self, c: u8) {
        unsafe {self.p.rhr_thr.write(c)}
    }

    pub fn read(&self) -> u8 {
        return self.p.rhr_thr.read()
    }

    pub fn get_ier(&self) -> u8 {
        return self.p.ier.read();
    }


}
