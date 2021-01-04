/// BubbleSortTest Implementation
/// 

use crate::benchmark::*;
use alloc::{
    vec,
    vec::Vec,
    string::String,
};

#[no_mangle]
pub struct BubbleSortTest {
    data: Vec<usize>
}

impl BenchMark<CpuTestErr> for BubbleSortTest {
    fn new() -> Self {
        Self {
            data: vec![2, 12, 14, 6, 13, 15, 16, 10, 0, 18, 11, 19, 9, 1, 7, 5, 4, 3, 8, 17]
        }
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::BuddleSortTestErr
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        for i in 0..self.data.len() {
            for j in 0..self.data.len() - i - 1 {
                if self.data[j] > self.data[j + 1] {
                    let temp = self.data[j];
                    self.data[j] = self.data[j + 1];
                    self.data[j + 1] = temp;
                }
            }
        }
        for i in 0..self.data.len() {
            xs_assert_eq!(self.data[i], i, self.err_type());
        }
        Ok(String::from("buddle_sort_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            self.data = vec![2, 12, 14, 6, 13, 15, 16, 10, 0, 18, 11, 19, 9, 1, 7, 5, 4, 3, 8, 17];
            for i in 0..self.data.len() {
                for j in 0..self.data.len() - i - 1 {
                    if self.data[j] > self.data[j + 1] {
                        let temp = self.data[j];
                        self.data[j] = self.data[j + 1];
                        self.data[j + 1] = temp;
                    }
                }
            }
            for i in 0..self.data.len() {
                xs_assert_eq!(self.data[i], i, self.err_type());
            }
        }
        Ok(String::from("buddle_sort_bench_test"))
    }
}