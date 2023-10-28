#![no_std]
#![no_main]

extern crate panic_halt;
use riscv_rt::entry;

#[entry]
fn main() -> !{
    let mut x = 0;
    loop {
        x += x+1;
    }
}
