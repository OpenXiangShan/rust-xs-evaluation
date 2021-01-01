/// AddU64Test Implementation

use crate::benchmark::{BenchMark, CpuTestErr, xs_assert_eq};
use crate::alloc::{
    vec::Vec,
    vec,
    string::String,
};

#[no_mangle]
pub struct AddU64Test {
    test_data: Vec<u64>,
    answer: Vec<u64>,
}

impl BenchMark for AddU64Test {
    fn new() -> Self {
        let mut test_data = Vec::new();
        test_data.push(0u64);
        test_data.push(1u64);
        test_data.push(2u64);
        test_data.push(0x7fff_ffff_ffff_ffffu64);
        test_data.push(0x8000_0000_0000_0000u64);
        test_data.push(0x8000_0000_0000_0001u64);
        test_data.push(0xffff_ffff_ffff_fffeu64);
        test_data.push(0xffff_ffff_ffff_ffffu64);
        let answer = vec![0u64, 0x1u64, 0x2u64, 0x7fffffffffffffffu64, 0x8000000000000000u64, 0x8000000000000001u64, 0xfffffffffffffffeu64, 0xffffffffffffffffu64, 0x1u64, 0x2u64, 0x3u64, 0x8000000000000000u64, 0x8000000000000001u64, 0x8000000000000002u64, 0xffffffffffffffffu64, 0u64, 0x2u64, 0x3u64, 0x4u64, 0x8000000000000001u64, 0x8000000000000002u64, 0x8000000000000003u64, 0u64, 0x1u64, 0x7fffffffffffffffu64, 0x8000000000000000u64, 0x8000000000000001u64, 0xfffffffffffffffeu64, 0xffffffffffffffffu64, 0u64, 0x7ffffffffffffffdu64, 0x7ffffffffffffffeu64, 0x8000000000000000u64, 0x8000000000000001u64, 0x8000000000000002u64, 0xffffffffffffffffu64, 0u64, 0x1u64, 0x7ffffffffffffffeu64, 0x7fffffffffffffffu64, 0x8000000000000001u64, 0x8000000000000002u64, 0x8000000000000003u64, 0u64, 0x1u64, 0x2u64, 0x7fffffffffffffffu64, 0x8000000000000000u64, 0xfffffffffffffffeu64, 0xffffffffffffffffu64, 0u64, 0x7ffffffffffffffdu64, 0x7ffffffffffffffeu64, 0x7fffffffffffffffu64, 0xfffffffffffffffcu64, 0xfffffffffffffffdu64, 0xffffffffffffffffu64, 0u64, 0x1u64, 0x7ffffffffffffffeu64, 0x7fffffffffffffffu64, 0x8000000000000000u64, 0xfffffffffffffffdu64, 0xfffffffffffffffeu64];
        Self {
            test_data,
            answer,
        }
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        let mut ans_index = 0;
        for i in 0..self.test_data.len() {
            for j in 0..self.test_data.len() {
                xs_assert_eq!(self.test_data[i] + self.test_data[j], self.answer[ans_index], self.err_type());
                ans_index += 1;
            }
        }
        Ok(String::from("addu64_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            let mut ans_index = 0;
            for i in 0..self.test_data.len() {
                for j in 0..self.test_data.len() {
                    xs_assert_eq!(self.test_data[i] + self.test_data[j], self.answer[ans_index], self.err_type());
                    ans_index += 1;
                }
            }
        }
        Ok(String::from("addu64_bench_test"))
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::AddU64TestErr
    }
}