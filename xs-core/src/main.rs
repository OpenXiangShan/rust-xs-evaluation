#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![allow(unused)]

extern crate xs_hal;

#[cfg(not(test))]
use core::alloc::Layout;
use core::panic;
#[cfg(not(test))]
use core::panic::PanicInfo;

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

// Ref: https://github.com/rcore-os/rCore-Tutorial/blob/master/os/src/entry.asm
global_asm!(
    "
        .section .text.entry
        .globl _start
    # 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
    _start:
        # 通过线性映射关系计算 boot_page_table 的物理页号
        lui t0, %hi(boot_page_table)
        li t1, 0xffffffff00000000
        sub t0, t0, t1
        srli t0, t0, 12
        # 8 << 60 是 satp 中使用 Sv39 模式的记号
        li t1, (8 << 60)
        or t0, t0, t1
        # 写入 satp 并更新 TLB
        csrw satp, t0
        sfence.vma
    
        # 加载栈的虚拟地址
        lui sp, %hi(boot_stack_top)
        addi sp, sp, %lo(boot_stack_top)
        # 跳转至 rust_main
        # 这里同时伴随 hart 和 dtb_pa 两个指针的传入（是 OpenSBI 帮我们完成的）
        lui t0, %hi(rust_main)
        addi t0, t0, %lo(rust_main)
        jr t0
    
        # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
        # 这里声明字段 .bss.stack 作为操作系统启动时的栈
        .section .bss.stack
        .global boot_stack
    boot_stack:
        # 16K 启动栈大小
        .space 4096 * 16
        .global boot_stack_top
    boot_stack_top:
        # 栈结尾
    
        # 初始内核映射所用的页表
        .section .data
        .align 12
        .global boot_page_table
    boot_page_table:
        # .8byte表示长度为8个字节的整数
        .8byte 0
        .8byte 0
        # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
        .8byte (0x80000 << 10) | 0xcf
        .zero 505 * 8
        # 第 508 项（外设用）：0xffff_ffff_0000_0000 -> 0x0000_0000，0xcf 表示 VRWXAD 均为 1
        .8byte (0x00000 << 10) | 0xcf
        .8byte 0
        # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
        .8byte (0x80000 << 10) | 0xcf
        .8byte 0
    "
);

// RustSBI enter here from M mode
#[no_mangle]
#[export_name = "rust_main"]
fn main(hart_id: usize, _dtb_pa: usize) -> ! {
    println!("hart {} enter XiangShan Core in S mode", hart_id);
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