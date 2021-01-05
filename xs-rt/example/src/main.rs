#![no_std]
#![no_main]
#![feature(llvm_asm)]

extern crate panic_halt;

use xs_rt::entry;

#[entry]
fn main() -> ! {
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(0) :: "volatile"); }
    loop {}
}