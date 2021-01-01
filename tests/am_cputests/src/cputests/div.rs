/// DivTest Implementation
/// 

use crate::benchmark::*;
#[allow(unused_imports)]
use crate::println;
use alloc::{
    string::String,
    vec::Vec,
};

const TEST_SIZE: usize = 10;

#[no_mangle]
pub struct DivTest {
    data: Vec<usize>,
}

impl BenchMark for DivTest {
    fn new() -> Self {
        let mut data = Vec::new();
        for i in 0..TEST_SIZE {
            data.push(i);
        }
        Self {
            data
        }
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::DivTestErr
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        for i in 0..self.data.len() {
            for j in 1..self.data.len() + 1 {
                self.data[i] *= j;
            }
        }
        for i in 0..self.data.len() {
            for j in 1..self.data.len() + 1 {
                self.data[i] /= j;
            }
            xs_assert_eq!(self.data[i], i, self.err_type());
        }
        Ok(String::from("div_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            for i in 0..self.data.len() {
                for j in 1..self.data.len() + 1 {
                    self.data[i] *= j;
                }
            }
            for i in 0..self.data.len() {
                for j in 1..self.data.len() + 1 {
                    self.data[i] /= j;
                }
                xs_assert_eq!(self.data[i], i, self.err_type());
            }
        }
        Ok(String::from("div_bench_test"))
    }
}

