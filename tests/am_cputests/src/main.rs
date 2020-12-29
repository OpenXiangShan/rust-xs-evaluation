#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate benchmark;
extern crate alloc;
extern crate bit;

mod cputests;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
// use linked_list_allocator::LockedHeap;
use buddy_system_allocator::LockedHeap;
use riscv::register::{
    mhartid,
};

use benchmark::BenchMark;
use cputests::{
    add::AddTest,
    bit::BitTest,
};

global_asm!(include_str!("entry.asm"));

const HEAP_SIZE: usize = 0x8000;


#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { 
        llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile");
    }
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    unsafe { 
        llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile");
    }
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
            fn _sheap();
            // fn _heap_size();
        }
        let sheap = &mut _sheap as *mut _ as usize;
        // let heap_size = &_heap_size as *const _ as usize;
        unsafe {
            r0::zero_bss(&mut _sbss, &mut _ebss);
            r0::init_data(&mut _sdata, &mut _edata, &_sidata);
            ALLOCATOR.lock().init(sheap, HEAP_SIZE);
        }
    }
    let mut add_test = AddTest::new();
    let mut bit_test = BitTest::new();
    add_test.test();
    bit_test.test();

    unsafe { 
        llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(0) :: "volatile");
    }
    loop {}
}
