//! Minimal startup / runtime for XiangShan CPU's
//!
//! Copyright 2021 Rust RISC-V team:https://github.com/rust-embedded/wg#the-riscv-team
//! # Minimum Supported Rust Version (MSRV)
//!
//! # Features
//!
//! This crate provides
//!
//! - Before main initialization of the `.bss` and `.data` sections.
//!
//! - `#[entry]` to declare the entry point of the program
//! - `#[pre_init]` to run code *before* `static` variables are initialized
//!
//! - A linker script that encodes the memory layout of a generic RISC-V
//!   microcontroller. This linker script is missing some information that must
//!   be supplied through a `memory.x` file (see example below). This file
//!   must be supplied using rustflags and listed *before* `link.x`. Arbitrary
//!   filename can be use instead of `memory.x`.
//!
//! - A `_sheap` symbol at whose address you can locate a heap.
//!
//! ``` text
//! $ cargo new --bin app && cd $_
//!
//! $ # add this crate as a dependency
//! $ edit Cargo.toml && cat $_
//! [dependencies]
//! xs-rt = { git = "https://github.com/RISCVERS/rust-xs-evaluation/tree/main/xs-hal" }
//!
//! $ # memory layout of the device
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   RAM : ORIGIN = 0x80000000, LENGTH = 128K
//! }
//!
//! REGION_ALIAS("REGION_TEXT", RAM);
//! REGION_ALIAS("REGION_RODATA", RAM);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", RAM);
//!
//! $ edit src/main.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! #![no_std]
//! #![no_main]
//! #![feature(alloc_error_handler)]
//! #![feature(llvm_asm)]
//! 
//! use xs_rt::entry;
//! use core::panic::PanicInfo;
//! 
//! #[panic_handler]
//! fn panic(_info: &PanicInfo) -> ! {
//!     unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(1) :: "volatile"); }
//!     // should not reach here
//!     loop {}
//! }
//! // use `main` as the entry point of this application
//! // `main` is not allowed to return
//! #[entry]
//! fn main() -> ! {
//!     // do something here
//!     loop { }
//! }
//! ```
//!
//! ``` text
//! $ mkdir .cargo && edit .cargo/config && cat $_
//! [target.riscv32imac-unknown-none-elf]
//! rustflags = [
//!   "-C", "link-arg=-Tmemory.x",
//!   "-C", "link-arg=-Tlink.x",
//! ]
//!
//! [build]
//! target = "riscv32imac-unknown-none-elf"
//! $ edit build.rs && cat $_
//! ```
//!
//! ``` ignore,no_run
//! use std::env;
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::Path;
//!
//! /// Put the linker script somewhere the linker can find it.
//! fn main() {
//!     let out_dir = env::var("OUT_DIR").expect("No out dir");
//!     let dest_path = Path::new(&out_dir);
//!     let mut f = File::create(&dest_path.join("memory.x"))
//!         .expect("Could not create file");
//!
//!     f.write_all(include_bytes!("memory.x"))
//!         .expect("Could not write file");
//!
//!     println!("cargo:rustc-link-search={}", dest_path.display());
//!
//!     println!("cargo:rerun-if-changed=memory.x");
//!     println!("cargo:rerun-if-changed=build.rs");
//! }
//! ```
//!
//! ``` text
//! $ cargo build --release
//!
//! $ riscv64-unknown-elf-objdump -Cd $(find target -name app) | head
//!
//! Disassembly of section .text:
//!
//! 0000000080000000 <_start>:
//!     80000000:	f1402673          	csrr	a2,mhartid
//!     80000004:	000002b7          	lui	t0,0x0
//!     80000008:	00028293          	mv	t0,t0

//! ```
//!
//! # Symbol interfaces
//!
//! This crate makes heavy use of symbols, linker sections and linker scripts to
//! provide most of its functionality. Below are described the main symbol
//! interfaces.
//!
//! ## `memory.x`
//!
//! This file supplies the information about the device to the linker.
//!
//! ### `MEMORY`
//!
//! The main information that this file must provide is the memory layout of
//! the device in the form of the `MEMORY` command. The command is documented
//! [here][2], but at a minimum you'll want to create at least one memory region.
//!
//! [2]: https://sourceware.org/binutils/docs/ld/MEMORY.html
//!
//! To support different relocation models (RAM-only, FLASH+RAM) multiple regions are used:
//!
//! - `REGION_TEXT` - for `.init`, `.trap` and `.text` sections
//! - `REGION_RODATA` - for `.rodata` section and storing initial values for `.data` section
//! - `REGION_DATA` - for `.data` section
//! - `REGION_BSS` - for `.bss` section
//! - `REGION_HEAP` - for the heap area
//! - `REGION_STACK` - for hart stacks
//!
//! Specific aliases for these regions must be defined in `memory.x` file (see example below).
//!
//! ### `_stext`
//!
//! This symbol provides the loading address of `.text` section. This value can be changed
//! to override the loading address of the firmware (for example, in case of bootloader present).
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_TEXT)`.
//!
//! ### `_stack_start`
//!
//! This symbol provides the address at which the call stack will be allocated.
//! The call stack grows downwards so this address is usually set to the highest
//! valid RAM address plus one (this *is* an invalid address but the processor
//! will decrement the stack pointer *before* using its value as an address).
//!
//! In case of multiple harts present, this address defines the initial stack pointer for hart 0.
//! Stack pointer for hart `N` is calculated as  `_stack_start - N * _hart_stack_size`.
//!
//! If omitted this symbol value will default to `ORIGIN(REGION_STACK) + LENGTH(REGION_STACK)`.
//!
//! #### Example
//!
//! Allocating the call stack on a different RAM region.
//!
//! ``` text
//! MEMORY
//! {
//!   L2_LIM : ORIGIN = 0x08000000, LENGTH = 1M
//!   RAM : ORIGIN = 0x80000000, LENGTH = 16K
//!   FLASH : ORIGIN = 0x20000000, LENGTH = 16M
//! }
//!
//! REGION_ALIAS("REGION_TEXT", FLASH);
//! REGION_ALIAS("REGION_RODATA", FLASH);
//! REGION_ALIAS("REGION_DATA", RAM);
//! REGION_ALIAS("REGION_BSS", RAM);
//! REGION_ALIAS("REGION_HEAP", RAM);
//! REGION_ALIAS("REGION_STACK", L2_LIM);
//!
//! _stack_start = ORIGIN(L2_LIM) + LENGTH(L2_LIM);
//! ```
//!
//! ### `_max_hart_id`
//!
//! This symbol defines the maximum hart id suppoted. All harts with id
//! greater than `_max_hart_id` will be redirected to `abort()`.
//!
//! This symbol is supposed to be redefined in platform support crates for
//! multi-core targets.
//!
//! If omitted this symbol value will default to 0 (single core).
//!
//! ### `_hart_stack_size`
//!
//! This symbol defines stack area size for *one* hart.
//!
//! If omitted this symbol value will default to 2K.
//!
//! ### `_heap_size`
//!
//! This symbol provides the size of a heap region. The default value is 0. You can set `_heap_size`
//! to a non-zero value if you are planning to use heap allocations.
//!
//! ### `_sheap`
//!
//! This symbol is located in RAM right after the `.bss` and `.data` sections.
//! You can use the address of this symbol as the start address of a heap
//! region. This symbol is 4 byte aligned so that address will be a multiple of 4.
//!
//! #### Example
//!
//! ``` no_run
//! extern crate some_allocator;
//!
//! extern "C" {
//!     static _sheap: u8;
//!     static _heap_size: u8;
//! }
//!
//! fn main() {
//!     unsafe {
//!         let heap_bottom = &_sheap as *const u8 as usize;
//!         let heap_size = &_heap_size as *const u8 as usize;
//!         some_allocator::initialize(heap_bottom, heap_size);
//!     }
//! }
//! ```
//!
//! ### `_mp_hook`
//!
//! This function is called from all the harts and must return true only for one hart,
//! which will perform memory initialization. For other harts it must return false
//! and implement wake-up in platform-dependent way (e.g. after waiting for a user interrupt).
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "_mp_hook"]
//! pub extern "Rust" fn mp_hook() -> bool {
//!    // ...
//! }
//! ```
//!
//! Default implementation of this function wakes hart 0 and busy-loops all the other harts.
//!
//! Default implementation of this function wakes hart 0 and busy-loops all the other harts.
//!
//! ### `ExceptionHandler`
//!
//! This function is called when exception is occured. The exception reason can be decoded from the
//! `mcause` register.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "ExceptionHandler"]
//! fn custom_exception_handler(trap_frame: &riscv_rt::TrapFrame) -> ! {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn ExceptionHandler(trap_frame: &riscv_rt::TrapFrame) -> ! {
//!     // ...
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.
//!
//!
//! ### Core interrupt handlers
//!
//! This functions are called when corresponding interrupt is occured.
//! You can define an interrupt handler with one of the following names:
//! * `UserSoft`
//! * `SupervisorSoft`
//! * `MachineSoft`
//! * `UserTimer`
//! * `SupervisorTimer`
//! * `MachineTimer`
//! * `UserExternal`
//! * `SupervisorExternal`
//! * `MachineExternal`
//!
//! For example:
//! ``` no_run
//! #[export_name = "MachineTimer"]
//! fn custom_timer_handler() {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn MachineTimer() {
//!     // ...
//! }
//! ```
//!
//! If interrupt handler is not explicitly defined, `DefaultHandler` is called.
//!
//! ### `DefaultHandler`
//!
//! This function is called when interrupt without defined interrupt handler is occured.
//! The interrupt reason can be decoded from the `mcause` register.
//!
//! This function can be redefined in the following way:
//!
//! ``` no_run
//! #[export_name = "DefaultHandler"]
//! fn custom_interrupt_handler() {
//!     // ...
//! }
//! ```
//! or
//! ``` no_run
//! #[no_mangle]
//! fn DefaultHandler() {
//!     // ...
//! }
//! ```
//!
//! Default implementation of this function stucks in a busy-loop.

// NOTE: Adapted from cortex-m/src/lib.rs
#![no_std]
#![deny(missing_docs)]
#![feature(alloc_error_handler)]
#![feature(llvm_asm, global_asm)]


extern crate r0;
extern crate riscv;
extern crate xs_rt_macros as macors;

pub use macors::{entry, pre_init};
use riscv::register::mcause;

global_asm!(
    ".section .init
    .globl _start
    _start:
        csrr    a2, mhartid
        lui     t0, %hi(_max_hart_id)
        add     t0, t0, %lo(_max_hart_id)
        bgtu    a2, t0, _start_abort
        la      sp, _stack_start
        lui     t0, %hi(_hart_stack_size)
        add     t0, t0, %lo(_hart_stack_size)
        beqz    a2, 2f  // Jump if single-hart
        mv      t1, a2
        mv      t2, t0
    1:
        add     t0, t0, t2
        addi    t1, t1, -1
        bnez    t1, 1b
    2:
        sub     sp, sp, t0
        csrw    mscratch, zero
        j _start_success
        
    _start_abort:
        wfi
        j _start_abort
    _start_success:
        j _start_rust"
);

#[export_name = "error: xs-rt appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

/// Rust entry point (_start_rust)
///
/// Zeros bss section, initializes data section and calls main. This function
/// never returns.
#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust() -> ! {
    #[rustfmt::skip]
    extern "Rust" {
        // This symbol will be provided by the user via `#[entry]`
        fn main() -> !;

        // This symbol will be provided by the user via `#[pre_init]`
        fn __pre_init();

        fn _setup_interrupts();

        fn _mp_hook() -> bool;
    }

    if _mp_hook() {
        __pre_init();

        r0::zero_bss(&mut _sbss, &mut _ebss);
        r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    }

    _setup_interrupts();

    main();
}

/// Registers saved in trap handler
#[allow(missing_docs)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
}

/// Trap entry point rust (_start_trap_rust)
///
/// `mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and ExceptionHandler
/// or one of the core interrupt handlers is called.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust(trap_frame: *const TrapFrame) {
    extern "C" {
        fn ExceptionHandler(trap_frame: &TrapFrame);
        fn DefaultHandler();
    }

    unsafe {
        let cause = mcause::read();
        if cause.is_exception() {
            ExceptionHandler(&*trap_frame)
        } else {
            let code = cause.code();
            if code < __INTERRUPTS.len() {
                let h = &__INTERRUPTS[code];
                if h.reserved == 0 {
                    DefaultHandler();
                } else {
                    (h.handler)();
                }
            } else {
                DefaultHandler();
            }
        }
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultExceptionHandler(trap_frame: &TrapFrame) -> ! {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultInterruptHandler() {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

/* Interrupts */
#[doc(hidden)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    MachineSoft,
    UserTimer,
    SupervisorTimer,
    MachineTimer,
    UserExternal,
    SupervisorExternal,
    MachineExternal,
}

pub use self::Interrupt as interrupt;

extern "C" {
    fn UserSoft();
    fn SupervisorSoft();
    fn MachineSoft();
    fn UserTimer();
    fn SupervisorTimer();
    fn MachineTimer();
    fn UserExternal();
    fn SupervisorExternal();
    fn MachineExternal();
}

#[doc(hidden)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[doc(hidden)]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 12] = [
    Vector { handler: UserSoft },
    Vector {
        handler: SupervisorSoft,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineSoft,
    },
    Vector { handler: UserTimer },
    Vector {
        handler: SupervisorTimer,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineTimer,
    },
    Vector {
        handler: UserExternal,
    },
    Vector {
        handler: SupervisorExternal,
    },
    Vector { reserved: 0 },
    Vector {
        handler: MachineExternal,
    },
];

global_asm!(
    "/*
    Trap entry point (_start_trap)

    Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
    restores caller saved registers and then returns.
*/
.section .trap
.global _start_trap
/* Make it .weak so PAC/HAL can provide their own if needed. */
.weak _start_trap

_start_trap:
    addi sp, sp, -16*8

    sd ra, 0*8(sp)
    sd t0, 1*8(sp)
    sd t1, 2*8(sp)
    sd t2, 3*8(sp)
    sd t3, 4*8(sp)
    sd t4, 5*8(sp)
    sd t5, 6*8(sp)
    sd t6, 7*8(sp)
    sd a0, 8*8(sp)
    sd a1, 9*8(sp)
    sd a2, 10*8(sp)
    sd a3, 11*8(sp)
    sd a4, 12*8(sp)
    sd a5, 13*8(sp)
    sd a6, 14*8(sp)
    sd a7, 15*8(sp)

    add a0, sp, zero
    jal ra, _start_trap_rust

    ld ra, 0*8(sp)
    ld t0, 1*8(sp)
    ld t1, 2*8(sp)
    ld t2, 3*8(sp)
    ld t3, 4*8(sp)
    ld t4, 5*8(sp)
    ld t5, 6*8(sp)
    ld t6, 7*8(sp)
    ld a0, 8*8(sp)
    ld a1, 9*8(sp)
    ld a2, 10*8(sp)
    ld a3, 11*8(sp)
    ld a4, 12*8(sp)
    ld a5, 13*8(sp)
    ld a6, 14*8(sp)
    ld a7, 15*8(sp)

    addi sp, sp, 16*8
    mret

.section .text
.global default_setup_interrupts

default_setup_interrupts:
    // Set trap handler
    la t0, _start_trap
    csrw mtvec, t0
    ret"
);



/// Func that run before main  
/// 
/// Default do nothing
/// The user of this crate can change the default implementation
/// by using the using the `#[pre_init]` attribute or creating a `__pre_init` function
#[no_mangle]
#[rustfmt::skip]
pub unsafe extern "Rust" fn default_pre_init() {}

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub extern "Rust" fn default_mp_hook() -> bool {
    use riscv::register::mhartid;
    match mhartid::read() {
        0 => true,
        _ => loop {
            unsafe { riscv::asm::wfi() }
        },
    }
}

