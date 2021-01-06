#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]


extern crate benchmark;
extern crate alloc;
extern crate bit;
extern crate xs_hal;
extern crate ansi_rgb;

mod cputests;
#[macro_use]
mod device;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use ansi_rgb::{ Foreground, red, green };
use buddy_system_allocator::LockedHeap;
use riscv::register::{mhartid};
use benchmark::ErrType;
use cputests::test_all;

use xs_hal::{XSPeripherals,  hit_trap};
use xs_rt::{entry, pre_init};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut XSPERIPHERALS: XSPeripherals = XSPeripherals::new(); 

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[xs] {}", info);
    hit_trap(1);
    // should not reach here
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    // oom hit the bad trap
    hit_trap(1);
    loop {}
}

extern "C" {
    static _sheap: u8;
    static _heap_size: u8;
}

#[pre_init]
unsafe fn before_main() {
    let heap_bottom = &_sheap as *const u8 as usize;
    let heap_size = &_heap_size as *const u8 as usize;
    // println!("[{}] heap_bottom: 0x{:x}, heap_size: 0x{:x}", "xs".fg(red()), heap_bottom, heap_size);
    ALLOCATOR.lock().init(heap_bottom, heap_size);
    device::init();
    device::print_logo();
    println!("[{}] XiangShan core {} is running", "xs".fg(red()), mhartid::read());
}


#[entry]
#[no_mangle]
fn main() -> ! {
    let results = test_all();
    
    let mut is_pass = true;
    for res in results.iter() {
        match res {
            Ok(string) => {
                println!("[xs] {} {}", string, "pass".fg(green()));
            },
            Err(err) => {
                println!("[xs] {} {}", err.as_str(), "failed".fg(red()));
                is_pass = false;
            }
        }
    }

    hit_trap(!is_pass as usize);
    loop {}
}
