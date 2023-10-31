#![no_std]
#![no_main]

extern crate panic_halt;
use riscv_rt::entry;
use numtoa::NumToA;
mod lib;

#[entry]
fn main() -> !{
    let mut uart = lib::Uart::new();
    let mut buf = [0u8;1024];
    let test_var:u32 = 0x20;
    test_var.numtoa_str(10, &mut buf);
    for x in buf {
        uart.write(x);
    }
    let mut a = uart.read();
    while a != 0x41 {
        a = uart.read();
        continue;
    }
    a.numtoa_str(10, &mut buf);
    for x in buf {
        uart.write(x);
    }

    loop {
    }
}
