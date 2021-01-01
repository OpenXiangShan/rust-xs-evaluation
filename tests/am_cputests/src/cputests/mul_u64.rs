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
    test_data: Vec<i32>,
    answer: Vec<i64>,
}

impl BenchMark for MulU64Test {
    fn new() -> Self {
        let test_data = vec![0xaeb1c2aa, 0x4500ff2b, 0x877190af, 0x11f42438];
        let answer = vec![0x19d29ab9db1a18e4, 0xea15986d3ac3088e, 0x2649e980fc0db236, 0xfa4c43da0a4a7d30, 0x1299898e2c56b139, 0xdf8123d50a319e65, 0x4d6dfa84c15dd68, 0x38c5d79b9e4357a1, 0xf78b91cb1efc4248, 0x14255a47fdfcc40];
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
                xs_assert_eq!(self.test_data[i] as i64 * self.test_data[j] as i64, self.answer[ans_index], self.err_type());
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
                    xs_assert_eq!((self.test_data[i] * self.test_data[j]) as i64, self.answer[ans_index], self.err_type());
                    ans_index += 1;
                }
            }
        }
        Ok(String::from("mulu64_bench_test"))
    }
}