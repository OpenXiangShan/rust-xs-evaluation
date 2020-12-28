#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
// use linked_list_allocator::LockedHeap;
use buddy_system_allocator::LockedHeap;
use riscv::register::{
    mhartid,
};

global_asm!(include_str!("entry.asm"));

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
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
            fn _head_size();
        }
        let sheap = &mut _sheap as *mut _ as usize;
        let heap_size = &_head_size as *const _ as usize;
        unsafe {
            r0::zero_bss(&mut _sbss, &mut _ebss);
            r0::init_data(&mut _sdata, &mut _edata, &_sidata);
            ALLOCATOR.lock().init(sheap, heap_size);
        }
    }
    let one = 1usize;
    let two  = 2usize;
    let three = 3usize;
    assert_eq!(one + two, three);
    loop {}
}
