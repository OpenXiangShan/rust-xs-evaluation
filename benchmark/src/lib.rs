//! BenchMark Abstract
//! 
#![no_std]
// #![feature(asm)]

extern crate alloc;

use alloc::string::String;

// Test error enum
#[no_mangle]
#[repr(C)]
pub enum CpuTestErr {
    AddTestErr,
    BitTestErr,
}

pub trait BenchMark {
    fn new() -> Self;
    fn single_test(&mut self) -> Result<String, CpuTestErr>;
    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr>;
    fn err_type(&self) -> CpuTestErr;
}

#[macro_export]
macro_rules! xs_assert_eq {
    ($left:expr, $right:expr, $z:expr) => {
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !((*left_val == *right_val) && (*right_val == *left_val)) {
                    return Err($z);
                } else {
                    // do nothing
                } 
            }
        }
    }
}

