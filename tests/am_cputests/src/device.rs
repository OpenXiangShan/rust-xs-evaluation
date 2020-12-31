//! XiangShan Device Implementation
//! 

use xs_hal::{XSPeripherals, UartLite};

pub fn init() -> &'static mut UartLite {
    let mut xs_peripherals = XSPeripherals::new();
    let uart_lite = xs_peripherals.take_uart_lite();
    uart_lite.init();
    uart_lite
}
