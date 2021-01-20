#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![allow(unused)]

extern crate xs_hal;
extern crate alloc;

#[cfg(not(test))]
use core::alloc::Layout;
use core::panic;
#[cfg(not(test))]
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;
use riscv::register::mhartid;
use alloc::{vec, vec::Vec};

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xs_hal::hit_trap(1);
    loop {}
}

#[cfg(not(test))]
#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    xs_hal::hit_trap(1);
    loop {}
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Ref: https://github.com/rcore-os/rCore-Tutorial/blob/master/os/src/entry.asm
global_asm!(
    "
    .section .text.entry
    .globl _start
_start:
    add t0, a0, 1
    slli t0, t0, 14
    // lui sp, %hi(boot_stack)
	la sp, boot_stack
    add sp, sp, t0

    // add by retrhelo, write tp reg 
    // csrr t1, mhartid 
    // mv tp, t1

    // lui t0, %hi(rust_main)
    // addi t0, t0, %lo(rust_main)
    // jr t0
    call rust_main

loop:
    j loop

    .section .bss.stack
    .align 12
    .globl boot_stack
boot_stack:
    .space 4096 * 4 * 2
    .globl boot_stack_top
boot_stack_top:
    "
);

// RustSBI enter here from M mode
#[export_name = "rust_main"]
fn main(hart_id: usize, _dtb_pa: usize) -> ! {
    println!("hart {} enter XiangShan Core in S mode", hart_id);

    extern "C" {
        static mut _sheap: u8;
        static _heap_size: u8;
    }
    
    if mhartid::read() == 0 {
        let sheap = unsafe { &mut _sheap } as *mut _ as usize;
        let heap_size = unsafe { &_heap_size } as *const u8 as usize;
        unsafe {
            ALLOCATOR.lock().init(sheap, heap_size);
        }
    }
    let mut data = Vec::new();
    for i in 0..5 {
        data.push(i);
    }
    assert_eq!(data, vec![0, 1, 2, 3, 4]);
    xs_hal::hit_trap(0);
    unreachable!()
}

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}

pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0);
}

use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point as usize);
            }
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::_print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! dbg {
    () => {
        println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($val:expr,) => { $crate::dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! dbgx {
    () => {
        println!("[{}:{}]", file!(), line!());
    };
    ($val:expr) => {
        match $val {
            tmp => {
                println!("[{}:{}] {} = {:#x?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($val:expr,) => { dbgx!($val) };
    ($($val:expr),+ $(,)?) => {
        ($(dbgx!($val)),+,)
    };
}