//! XiangShan Hal Implementation
//! 香山目前只有 UartLite 可用于 embedded-hal 实现
#![no_std]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]
#![feature(llvm_asm)]

extern crate core;
extern crate tock_registers;
extern crate embedded_hal;
extern crate nb;

// use core::mem::replace;
use core::{convert::Infallible, fmt::{self, Write}};
#[allow(unused_imports)]
use tock_registers::{
    register_structs,
    register_bitfields,
    registers::{ReadOnly, ReadWrite},
};
use ansi_rgb::{Foreground, blue};

const UARTLITE_MMIO: usize = 0x4060_0000;
// const UARTLITE_RST_FIFO: u8 = 0x03;
// const UARTLITE_TX_FULL: u8 = 0x08;
// const UARTLITE_RX_VALID: u8 = 0x01;

const CLINT_MMIO: usize = 0x3800_0000;
    
register_structs! {
    /// UartLite MMIO
    /// |offset|register|description|
    /// |---|---|---|
    /// |0h|Rx FIFO|receive data fifo|
    /// |04h|Tx FIFO|send data fifo|
    /// |08h|status reg|IP 核状态寄存器|
    /// |0ch|control reg|IP 核控制寄存器|
    pub UartLite {
        (0x00 => rx_fifo: ReadOnly<u32>),
        (0x04 => tx_fifo: ReadWrite<u32>),
        // [0 => Rx Vaild]
        // [1 => Rx Full]
        // [2 => Tx Empty]
        // [3 => Tx Full]
        // [4 => Intr Enabled]
        // [5 => Overrun Error]
        // [6 => Frame Error]
        // [7 => Parity Error]
        // [8..31 => Reserved]
        (0x08 => stat_reg: ReadOnly<u32, Status::Register>),
        // [0 => Rst Tx FIFO]
        // [1 => Rst Rx FIFO]
        // [2..3 => Reserved]
        // [4 => Enable Intr]
        // [5..31 => Reserved]
        (0x0c => ctrl_reg: ReadWrite<u32, Control::Register>),
        (0x10 => @END),
    },
    /// Clint MMIO
    pub Clint {
        (0x0000 => msip: [ReadWrite<u32>; 2]),
        (0x0008 => _reserved_0),
        (0x4000 => mtimecmp: [ReadWrite<u64>; 2]),
        (0x4010 => _reserved_1),
        (0xBFF8 => mtime: ReadOnly<u64>),
        (0xC000 => @END),
    }
}

register_bitfields! [
    u32,
    Control [
        RST_TX OFFSET(0) NUMBITS(1) [],
        RST_RX OFFSET(1) NUMBITS(1) [],
        ENABLE_INTR OFFSET(4) NUMBITS(1) []
    ],
    Status [
        RX_VALID OFFSET(0) NUMBITS(1) [],
        RX_FULL OFFSET(1) NUMBITS(1) [],
        TX_EMPTY OFFSET(2) NUMBITS(1) [],
        TX_FULL OFFSET(3) NUMBITS(1) [],
        INTR_ENABLED OFFSET(4) NUMBITS(1) [],
        OVERRUN OFFSET(5) NUMBITS(1) [],
        FRAME_ERROR OFFSET(6) NUMBITS(1) [],
        PARITY_ERROR OFFSET(7) NUMBITS(1) []
    ]
];

impl UartLite {
    pub fn new() -> &'static mut UartLite {
        unsafe { &mut *(UARTLITE_MMIO as *mut UartLite) }
    }

    pub fn init(&mut self) {
        self.ctrl_reg.write(Control::RST_TX.val(1));
        self.ctrl_reg.write(Control::RST_RX.val(1));
    }

    pub fn putchar(&mut self, ch: char) {
        while self.stat_reg.is_set(Status::TX_FULL) {}
        self.tx_fifo.set(ch as u32);
    }

    pub fn getchar(&self) -> Result<u8, ()> {
        match self.stat_reg.is_set(Status::RX_VALID) {
            true => Ok(self.rx_fifo.get() as u8),
            false => Err(()),
        } 
    }
}

use embedded_hal::serial;

impl serial::Read<u8> for UartLite {
    type Error = Infallible;
    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        match self.stat_reg.is_set(Status::RX_VALID) {
            true => Ok(self.rx_fifo.get() as u8),
            false => Err(nb::Error::WouldBlock),
        }
    }
}

impl serial::Write<u8> for UartLite {
    type Error = Infallible;
    fn try_write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        match self.stat_reg.is_set(Status::TX_FULL) {
            true => Err(nb::Error::WouldBlock),
            false => {
                self.tx_fifo.set(word as u32);
                Ok(())
            }
        }
    }
    
    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        // 这里本来应该是要根据状态寄存器的 `TX_EMPTY` 位
        // 判断数据是否已经全部发出去
        // 但是由于当前香山的仿真环境中不会对状态寄存器和控制寄存器
        // 做任何事情，因此直接返回 `Ok(())`
        // match self.stat_reg.is_set(Status::TX_EMPTY) {
        //     true => Ok(()),
        //     false => Err(nb::Error::WouldBlock),
        // }
        Ok(())
    }
}

/// Clint should not in embedded-hal implementation
/// TODO: remove it
impl Clint {
    pub unsafe fn new() -> &'static mut Clint {
        &mut *(CLINT_MMIO as *mut Clint)
    } 

    pub fn get_mtime(&self) -> u64 {
        self.mtime.get()
    }

    pub fn set_timer(&mut self, hart_id: usize, instant: u64) {
        self.mtimecmp[hart_id].set(instant);
    }

    pub fn send_soft(&mut self, hart_id: usize) {
        self.msip[hart_id].set(1);
    }

    pub fn clear_soft(&mut self, hart_id: usize) {
        self.msip[hart_id].set(0);
    }
}


// TODO: find a better to abstract peripherals
// pub struct XSPeripherals {
//     uart_lite: Option<usize>    // base adderss of mmio register
// }

// impl XSPeripherals {
//     pub const fn new() -> Self {
//         Self {
//             uart_lite: Some(UARTLITE_MMIO)
//         }
//     }

//     pub fn take_uart_lite(&mut self) -> &'static mut UartLite {
//         let uart = replace(&mut self.uart_lite, None).unwrap();
//         unsafe { &mut *(uart as *mut UartLite) }
//     }

//     pub fn release_uart_lite(&mut self) {
//         let _uart = replace(&mut self.uart_lite, Some(UARTLITE_MMIO));
//     }
// }

// The code below should not in embedded-hal implementation
// TODO: remove it
pub fn hit_trap(trapcode: usize) {
    unsafe { llvm_asm!("mv a0, $0; .word 0x0005006b" :: "r"(trapcode) :: "volatile"); }
}

pub fn puts(s: &str) {
    let uart_lite = UartLite::new();
    for ch in s.chars() {
        uart_lite.putchar(ch);
    }
}

struct XSStdout;

impl fmt::Write for XSStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    XSStdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        _print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => (
        $crate::print!("\n")
    );
    ($($arg:tt)*) => (
        $crate::print!("{}\n", format_args!($($arg)*))
    )
}

pub fn print_logo() {
    println!("{}", " ___  ___    ________  ".fg(blue()));
    println!("{}", "|\"  \\/\"  |  /\"       ) ".fg(blue()));
    println!("{}", " \\   \\  /  (:   \\___/  ".fg(blue()));
    println!("{}", "  \\  \\/    \\___  \\    ".fg(blue()));
    println!("{}", "  /\\.  \\     __/  \\\\   ".fg(blue()));
    println!("{}", " /  \\   \\   /\" \\   :)  ".fg(blue()));
    println!("{}", "|___/\\___| (_______/   ".fg(blue()));
}

