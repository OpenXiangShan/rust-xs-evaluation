//! XiangShan Hal Implementation
#![no_std]
#![feature(const_fn)]
#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]

extern crate core;
extern crate tock_registers;

use core::mem::replace;
#[allow(unused_imports)]
use tock_registers::{
    register_structs,
    register_bitfields,
    registers::{ReadOnly, ReadWrite},
};

const UARTLITE_MMIO: usize = 0x4060_0000;
// const UARTLITE_RST_FIFO: u8 = 0x03;
// const UARTLITE_TX_FULL: u8 = 0x08;
// const UARTLITE_RX_VALID: u8 = 0x01;

register_structs! {
    pub UartLite {
        (0x00 => rx_fifo: ReadOnly<u32>),
        (0x04 => tx_fifo: ReadWrite<u32>),
        (0x08 => stat_reg: ReadOnly<u32, Status::Register>),
        (0x0c => ctrl_reg: ReadWrite<u32, Control::Register>),
        (0x10 => @END),
    }
}

register_bitfields! [
    u32,
    Control [
        RST_FIFO OFFSET(0) NUMBITS(2) []
    ],
    Status [
        RX_VALID OFFSET(1) NUMBITS(1) [],
        TX_FULL OFFSET(8) NUMBITS(1) []
    ]
];

impl UartLite {
    pub fn init(&mut self) {
        self.ctrl_reg.write(Control::RST_FIFO.val(3));
    }

    pub fn putchar(&mut self, ch: char) {
        if ch == '\n' {
            self.putchar('\r');
        }
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

pub struct XSPeripherals {
    uart_lite: Option<&'static mut UartLite>
}

impl XSPeripherals {
    pub const fn new() -> Self {
        Self {
            uart_lite: unsafe { Some(&mut *(UARTLITE_MMIO as *mut UartLite)) }
        }
    }

    pub fn take_uart_lite(&mut self) -> &'static mut UartLite {
        let uart = replace(&mut self.uart_lite, None);
        uart.unwrap()
    }
}
