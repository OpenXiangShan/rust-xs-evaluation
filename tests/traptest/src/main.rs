#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate alloc;
extern crate xs_hal;
extern crate ansi_rgb;

#[macro_use]
mod device;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;
use ansi_rgb::{ Foreground, red};
#[allow(unused_imports)]
use riscv::{asm::wfi, register::{mhartid, mie, mip, mstatus, time}};
use xs_hal::{XSPeripherals, hit_trap, Clint};
use xs_rt::{entry, pre_init};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

static mut XSPERIPHERALS: XSPeripherals = XSPeripherals::new(); 

const INTERVAL: u64 = 390000000 / 200;

static  mut COUNTER: usize = 0;
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
    // TODO
}

#[entry]
#[no_mangle]
fn main() -> ! {
    unsafe {
        let heap_bottom = &_sheap as *const u8 as usize;
        let heap_size = &_heap_size as *const u8 as usize;
        ALLOCATOR.lock().init(heap_bottom, heap_size);
    }
    device::init();
    device::print_logo();
    println!("[{}] XiangShan core {} is running", "xs".fg(red()), mhartid::read());
    unsafe {
        // The MTIP bit is read-only and is cleared by writing to the memory-mapped machine-mode timer compare register
        // mip::set_mtimer(); 
        mie::set_mtimer();
        // TODO: PC will block here
        // mstatus::set_mie();
        let clint = Clint::new();
        clint.set_timer(mhartid::read(), clint.get_mtime() + INTERVAL);
    }
    // unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(!is_pass) :: "volatile"); }
    // hit_trap(0);
    loop {}
}

#[export_name = "MachineTimer"]
fn mtimer_handler() {
    unsafe {
        let clint = Clint::new();
        clint.set_timer(mhartid::read(), clint.get_mtime() + INTERVAL);
        COUNTER += 1;
        if COUNTER % 10 == 0 {
            println!("[xs] timer interrupt! counter: {}", COUNTER);
        }
    }
}

#[export_name = "_mp_hook"]
fn mp_hook() -> bool {
    let hart_id = mhartid::read();
    match hart_id {
        0 => true,
        _ => {            
            unsafe {
                let clint  = Clint::new();
                clint.clear_soft(hart_id);
                mie::set_msoft();
                loop {
                    wfi();
                    if mip::read().msoft() {break;}
                }
                mie::clear_msoft();
                clint.clear_soft(hart_id);
            }
            false
        }
    }
}