//! BenchMark Abstract
//! 
#![no_std]
#![feature(asm)]

pub trait BenchMark {
    fn new() -> Self;
    fn test(&self);
}
