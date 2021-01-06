# Minimal startup / runtime for XiangShan CPU's
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)  
Ref Rust RISC-V team:https://github.com/rust-embedded/wg#the-riscv-team  

## Minimum Supported Rust Version (MSRV)

## Features
This crate provides  
- Before main initialization of the `.bss` and `.data` sections.
- `#[entry]` to declare the entry point of the program
- `#[pre_init]` to run code *before* `static` variables are initialized
- A linker script that encodes the memory layout of a generic RISC-V
  microcontroller. This linker script is missing some information that must
  be supplied through a `memory.x` file (see example below). This file
  must be supplied using rustflags and listed *before* `link.x`. Arbitrary
  filename can be use instead of `memory.x`.
- A `_sheap` symbol at whose address you can locate a heap.

简单来说，就是通过这个包只需要几行代码就可以搭建香山的最小 Rust 运行时环境。  
具体请阅读源码 `src/lib.rs` 注释。  

## Example
```Rust
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(llvm_asm)]

use xs_rt::entry;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> {
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile"); }
    // should not reach here
    loop {}
}
// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> {
    // do something here
    loop { }
}
```

## TODO
+ ExceptionHandler
+ InterruptHandler
+ Trapframe
+ 页表

## License
MIT License  

