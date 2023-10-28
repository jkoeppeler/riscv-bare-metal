#![no_std]
#![no_main]

extern crate panic_halt;
use riscv_rt::entry;
mod lib;

#[entry]
fn main() -> !{
    let mut uart = lib::Uart::new();
    loop {
        uart.write(0x41);
    }
}
