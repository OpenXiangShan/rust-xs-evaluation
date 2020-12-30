/// AddTest Implementation

use crate::benchmark::{BenchMark, CpuTestErr, xs_assert_eq};
use crate::alloc::{
    vec::Vec,
    string::String,
};

const TEST_SIZE: usize = 20;

#[no_mangle]
pub struct AddTest {
    list_0: Vec<usize>,
    list_1: Vec<usize>,
    result: Vec<usize>,
}

impl BenchMark for AddTest {
    fn new() -> Self {
        let mut list_0: Vec<usize> = Vec::new();
        let mut list_1: Vec<usize> = Vec::new();
        let mut result: Vec<usize> = Vec::new();
        for i in 0..TEST_SIZE {
            list_0.push(i);
            list_1.push(i + 1);
            result.push((i << 1) + 1);
        }
        Self {
            list_0,
            list_1,
            result,
        }
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        xs_assert_eq!(self.list_0.len(), self.list_1.len(), self.err_type());
        xs_assert_eq!(self.list_0.len(), self.result.len(), self.err_type());
        for i in 0..self.result.len() {
            xs_assert_eq!(self.list_0[i] + self.list_1[i], self.result[i], self.err_type());
            self.list_0[i] = 0xffff_ffff_ffff_ffff;
            self.list_1[i] = 0x1;
            self.result[i] = 0x0;
            xs_assert_eq!(self.result[i], self.list_0[i] + self.list_1[i], self.err_type());
        }   
        Ok(String::from("add_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        xs_assert_eq!(self.list_0.len(), self.list_1.len(), self.err_type());
        xs_assert_eq!(self.list_0.len(), self.result.len(), self.err_type());
        for _ in 0..bench_size {
            for i in 0..self.result.len() {
                xs_assert_eq!(self.list_0[i] + self.list_1[i], self.result[i], self.err_type());
                self.list_0[i] = 0xffff_ffff_ffff_ffff;
                self.list_1[i] = 0x1;
                self.result[i] = 0x0;
                xs_assert_eq!(self.result[i], self.list_0[i] + self.list_1[i], self.err_type());
            }   
        }
        Ok(String::from("add_bench_test"))
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::AddTestErr
    }
}