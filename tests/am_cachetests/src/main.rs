#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate benchmark;
extern crate alloc;
extern crate xs_hal;
extern crate ansi_rgb;

mod cachetests;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;
use ansi_rgb::{ Foreground, red, green };
use riscv::register::{mhartid};
use benchmark::ErrType;
use xs_hal::{hit_trap, _print, println, print_logo, UartLite};
use xs_rt::{entry, pre_init};
use cachetests::test_all;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

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
    let uart_lite = UartLite::new();
    uart_lite.init();
}

#[entry]
#[no_mangle]
fn main() -> ! {
    unsafe {
        let heap_bottom = &_sheap as *const u8 as usize;
        let heap_size = &_heap_size as *const u8 as usize;
        ALLOCATOR.lock().init(heap_bottom, heap_size);
    }
    print_logo();
    println!("[{}] XiangShan core {} is running", "xs".fg(red()), mhartid::read());
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
    // unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(!is_pass) :: "volatile"); }
    hit_trap(!is_pass as usize);
    loop {}
}
