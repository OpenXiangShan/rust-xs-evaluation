#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate buddy_system_allocator;

#[cfg(not(test))]
use core::{
    alloc::Layout,
    panic::PanicInfo,
};
use alloc::vec;
use xs_rt::{entry, pre_init};
use buddy_system_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

extern "C" {
    static _sheap: u8;
    static _heap_size: u8;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // should not reach here
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    // oom hit the bad trap
    loop {}
}

#[pre_init]
unsafe fn allocator_init() {
    let heap_bottom = &_sheap as *const u8 as usize;
    let heap_size = &_heap_size as *const u8 as usize;
    ALLOCATOR.lock().init(heap_bottom, heap_size);
}

#[entry]
fn main() -> ! {
    let mut data = vec![1, 2, 3, 4, 5];
    data.push(6);
    data.push(7);
    data.push(8);
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(0) :: "volatile"); }
    loop {}
}

