#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate benchmark;
extern crate alloc;
extern crate bit;
extern crate xs_hal;

mod cputests;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;
use riscv::register::{
    mhartid,
};
use alloc::vec::Vec;

use benchmark::BenchMark;
use xs_hal::XSPeripherals;

use cputests::{
    add::AddTest,
    bit::BitTest,
};

global_asm!(include_str!("entry.asm"));

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut XSPERIPHERALS: XSPeripherals = XSPeripherals::new();
const BENCH_SIZE: usize = 20;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // panic, hit the bad trap
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile"); }
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    // oom hit the bad trap
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile"); }
    loop {}
}


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    if mhartid::read() == 0 {
        extern "C" {
            static mut _ebss: u32;
            static mut _sbss: u32;
            static mut _edata: u32;
            static mut _sdata: u32;
            static _sidata: u32;
            static _sheap: u8;
            static _heap_size: u8;
        }
        
        unsafe {
            let sheap = & _sheap as *const u8 as usize;
            let heap_size = &_heap_size as *const u8 as usize;
            r0::zero_bss(&mut _sbss, &mut _ebss);
            r0::init_data(&mut _sdata, &mut _edata, &_sidata);
            ALLOCATOR.lock().init(sheap, heap_size);
        }
        let _uart_lite = XSPERIPHERALS.take_uart_lite();

    }
    let mut results = Vec::new();
    let mut add_test = AddTest::new();
    let mut bit_test = BitTest::new();
    results.push(add_test.single_test());
    results.push(add_test.bench_test(BENCH_SIZE));
    results.push(bit_test.single_test());
    results.push(bit_test.bench_test(BENCH_SIZE));
    let mut is_pass = true;
    for res in results.iter() {
        match res {
            Ok(_string) => {
                // TODO
            },
            Err(_err) => {
                is_pass = false;
            }
        }
    }

    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(!is_pass) :: "volatile"); }
    loop {}
}
