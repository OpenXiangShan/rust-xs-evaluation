/// MulU64Test Implementation
/// 

use crate::benchmark::*;
use alloc::{
    vec,
    vec::Vec,
    string::String,
};

#[no_mangle]
pub struct MulU64Test {
    test_data: Vec<u32>,
    answer: Vec<u64>,
}

impl BenchMark for MulU64Test {
    fn new() -> Self {
        let test_data = vec![0xaeb1c2aa, 0x4500ff2b, 0x877190af, 0x11f42438];
        let answer = vec![0x19d29ab9db1a18e4u64, 0xea15986d3ac3088eu64, 0x2649e980fc0db236u64, 0xfa4c43da0a4a7d30u64, 0x1299898e2c56b139u64, 0xdf8123d50a319e65u64, 0x4d6dfa84c15dd68u64, 0x38c5d79b9e4357a1u64, 0xf78b91cb1efc4248u64, 0x14255a47fdfcc40u64];
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
            for j in 0..self.test_data.len() {
                xs_assert_eq!((self.test_data[i] * self.test_data[j]) as u64, self.answer[ans_index], self.err_type());
                ans_index += 1;
            }
        }
        Ok(String::from("mulu64_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            let mut ans_index = 0;
            for i in 0..self.test_data.len() {
                for j in 0..self.test_data.len() {
                    xs_assert_eq!((self.test_data[i] * self.test_data[j]) as u64, self.answer[ans_index], self.err_type());
                    ans_index += 1;
                }
            }
        }
        Ok(String::from("mulu64_bench_test"))
    }
}