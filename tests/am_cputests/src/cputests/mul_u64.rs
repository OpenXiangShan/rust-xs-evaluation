/// MulU64Test Implementation
/// 

use crate::benchmark::*;
use crate::println;
use alloc::{
    vec,
    vec::Vec,
    string::String,
};

#[no_mangle]
pub struct MulU64Test {
    test_data: Vec<u64>,
    answer: Vec<u64>,
}

impl BenchMark for MulU64Test {
    fn new() -> Self {
        let test_data = vec![0xaeb1c2aa, 0x4500ff2b, 0x877190af, 0x11f42438];
        let answer = vec![0x7736200ddb1a18e4, 0x2f1697983ac3088e, 0x5c6d3cd9fc0db236, 0xc4068120a4a7d30, 0x1299898e2c56b139, 0x248223000a319e65, 0x4d6dfa84c15dd68, 0x47a8f8f99e4357a1, 0x97fb6031efc4248, 0x14255a47fdfcc40];
        Self {
            test_data,
            answer,
        }
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::MulU64TestErr
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        let mut ans_index = 0;
        for i in 0..self.test_data.len() {
            for j in i..self.test_data.len() {
                println!("0x{:x} * 0x{:x} = 0x{:x}, answer: 0x{:x}", self.test_data[i], self.test_data[j], self.test_data[i] * self.test_data[j], self.answer[ans_index]);
                xs_assert_eq!(self.test_data[i] * self.test_data[j], self.answer[ans_index], self.err_type());
                ans_index += 1;
            }
        }
        Ok(String::from("mulu64_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            let mut ans_index = 0;
            for i in 0..self.test_data.len() {
                for j in i..self.test_data.len() {
                    xs_assert_eq!(self.test_data[i] * self.test_data[j], self.answer[ans_index], self.err_type());
                    ans_index += 1;
                }
            }
        }
        Ok(String::from("mulu64_bench_test"))
    }
}