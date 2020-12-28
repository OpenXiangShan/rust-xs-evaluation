#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use riscv::register::{
    mhartid,

};


#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(layout: Layout) -> ! {
    loop {}
}

#[export_name = "_start"]
#[link_section = ".text.entry"] // this is stable
fn main() -> ! {
    unsafe {
        llvm_asm!(
            "
        csrr    a2, mhartid
        lui     t0, %hi(_max_hart_id)
        add     t0, t0, %lo(_max_hart_id)
        bgtu    a2, t0, _start_abort
        la      sp, _stack_start
        lui     t0, %hi(_hart_stack_size)
        add     t0, t0, %lo(_hart_stack_size)
    .ifdef __riscv_mul
        mul     t0, a2, t0
    .else
        beqz    a2, 2f  // Jump if single-hart
        mv      t1, a2
        mv      t2, t0
    1:
        add     t0, t0, t2
        addi    t1, t1, -1
        bnez    t1, 1b
    2:
    .endif
        sub     sp, sp, t0
        csrw    mscratch, zero
        j _start_success
        
    _start_abort:
        wfi
        j _start_abort
    _start_success:
        
    "
        )
    };

    if mhartid::read() == 0 {
        extern "C" {
            fn _sheap();
            fn _head_size();
        }
        let sheap = &mut _sheap as *mut _ as usize;
        let heap_size = &_head_size as *const _ as usize;
        unsafe {
            ALLOCATOR.lock().init(sheap, heap_size);
        }
    }
    let one = 1usize;
    let two  = 2usize;
    let three = 3usize;
    assert_eq!(one + two, three);
    loop {}
}
