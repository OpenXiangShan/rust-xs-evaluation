//! XiangShan Device Implementation
//! 

use crate::XSPERIPHERALS;
use core::fmt::{self, Write};
use ansi_rgb::{Foreground, blue};

pub fn init() {
    let uart_lite = unsafe { XSPERIPHERALS.take_uart_lite() };
    uart_lite.init();
    unsafe { XSPERIPHERALS.release_uart_lite(); }
}

pub fn puts(s: &str) {
    let uart_lite = unsafe { XSPERIPHERALS.take_uart_lite() };
    for ch in s.chars() {
        uart_lite.putchar(ch);
    }
    unsafe { XSPERIPHERALS.release_uart_lite(); }
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
        $crate::device::_print(format_args!($($arg)*));
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


// ___  ___    ________  
// |"  \/"  |  /"       ) 
//  \   \  /  (:   \___/  
//   \\  \/    \___  \    
//   /\.  \     __/  \\   
//  /  \   \   /" \   :)  
// |___/\___| (_______/   
                       
pub fn print_logo() {
    println!("{}", " ___  ___    ________  ".fg(blue()));
    println!("{}", "|\"  \\/\"  |  /\"       ) ".fg(blue()));
    println!("{}", " \\   \\  /  (:   \\___/  ".fg(blue()));
    println!("{}", "  \\  \\/    \\___  \\    ".fg(blue()));
    println!("{}", "  /\\.  \\     __/  \\\\   ".fg(blue()));
    println!("{}", " /  \\   \\   /\" \\   :)  ".fg(blue()));
    println!("{}", "|___/\\___| (_______/   ".fg(blue()));
}
