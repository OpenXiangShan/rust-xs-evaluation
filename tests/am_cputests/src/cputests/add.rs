/// AddTest Implementation

use crate::benchmark::BenchMark;
use crate::alloc::vec::Vec;

const TEST_SIZE: usize = 20;
const BATCH_SIZE: usize = 20;

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
    fn test(&mut self) {
        assert_eq!(self.list_0.len(), self.list_1.len());
        assert_eq!(self.list_0.len(), self.result.len());
        for _ in 0..BATCH_SIZE {
            for i in 0..self.result.len() {
                assert_eq!(self.list_0[i] + self.list_1[i], self.result[i]);
            }
        }
        self.list_0[0] = 0xffff_ffff_ffff_ffff;
        self.list_1[0] = 0x1;
        self.result[0] = 0x0;
        assert_eq!(self.result[0], self.list_0[0] + self.list_1[0]);
    }
}