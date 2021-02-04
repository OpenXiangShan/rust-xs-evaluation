#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(global_asm)]
#![feature(llvm_asm)]

extern crate alloc;
extern crate xs_hal;
extern crate ansi_rgb;
extern crate async_rt;

#[cfg(not(test))]
use core::alloc::Layout;
#[cfg(not(test))]
use core::panic::PanicInfo;
use buddy_system_allocator::LockedHeap;
use ansi_rgb::{ Foreground, red};
#[allow(unused_imports)]
use riscv::{asm::wfi, register::{mhartid, mie, mip, mstatus, time}};
use xs_hal::{hit_trap, Clint, UartLite, _print, println, print_logo};
use xs_rt::{entry, pre_init};
use core::future::Future;
use core::{
    pin::Pin,
    task::{Context, Poll},
};

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

struct Foo {}

impl Future for Foo {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

fn a() -> impl Future<Output = ()> {
    println!("[a] hello world");
    Foo {}
}

fn b() -> impl Future<Output = ()> {
    println!("[b] hello world");
    Foo {}
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

    async_rt::run(async {
        b().await;
        a().await;
    });
    async_rt::block(async {
        a().await;
    });
    
    hit_trap(0);
    unreachable!()
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