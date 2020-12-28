#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate benchmark;
extern crate alloc;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use alloc::vec::Vec;
// use linked_list_allocator::LockedHeap;
use buddy_system_allocator::LockedHeap;
use riscv::register::{
    mhartid,
};
use benchmark::BenchMark;

global_asm!(include_str!("entry.asm"));

const TEST_SIZE: usize = 20;
const BATCH_SIZE: usize = 10;

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
struct AddTest {
    list_0: Vec<usize>,
    list_1: Vec<usize>,
    result: Vec<usize>,
}

impl BenchMark for AddTest {
    fn new() -> Self {
        let mut list_0: Vec<usize> = Vec::new();
        let mut list_1: Vec<usize> = Vec::new();
        let mut result: Vec<usize> = Vec::new();
        for i in 0..TEST_SIZE {
            list_0.push(i);
            list_1.push(i + 1);
            result.push(i << 1 + 1);
        }
        Self {
            list_0,
            list_1,
            result,
        }
    }
    fn test(&self) {
        assert_eq!(self.list_0.len(), self.list_1.len());
        assert_eq!(self.list_0.len(), self.result.len());
        for _ in 0..BATCH_SIZE {
            for i in 0..self.result.len() {
                assert_eq!(self.list_0[i] + self.list_1[i], self.result[i]);
            }
        }
    }
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
    let add_test = AddTest::new();
    add_test.test();
    loop {}
}
