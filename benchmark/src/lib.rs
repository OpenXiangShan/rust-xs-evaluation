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
    AddU64TestErr,
    BuddleSortTestErr,
    MulU64TestErr,
    DivTestErr,
    LoadStoreTestErr,
}

impl CpuTestErr {
    pub fn as_str(&self) -> &str {
        match self {
            CpuTestErr::AddTestErr => "add test error",
            CpuTestErr::BitTestErr => "bit test error",
            CpuTestErr::AddU64TestErr => "addu64 test error",
            CpuTestErr::BuddleSortTestErr => "buddle sort test error",
            CpuTestErr::MulU64TestErr => "mulu64 test error",
            CpuTestErr::DivTestErr => "div test error",
            CpuTestErr::LoadStoreTestErr => "load store error",
        }
    }
}

#[no_mangle]
#[repr(C)]
pub enum CacheTestErr {
    AccessTestErr,
}

impl CacheTestErr {
    pub fn as_str(&self) -> &str {
        match self {
            CacheTestErr::AccessTestErr => "access test error",
        }
    }
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

