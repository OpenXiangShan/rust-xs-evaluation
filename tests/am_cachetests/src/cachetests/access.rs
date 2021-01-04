/// AccessTest Implementation
/// 

use crate::benchmark::{BenchMark, CacheTestErr, xs_assert_eq};
use crate::alloc::{
    string::String,
};

const N: usize = 64;
const CACHE_SIZE: usize = 32 * 1024;

#[no_mangle]
pub struct AccessTest {
    ans: [u32; 2 * CACHE_SIZE  / 4]
}

impl BenchMark<CacheTestErr> for AccessTest {
    fn new() -> Self {
        let ans = [0u32; 2 * CACHE_SIZE / 4];
        Self { ans }
    }

    fn err_type(&self) -> CacheTestErr {
        CacheTestErr::AccessTestErr
    }
    
    fn single_test(&mut self) -> Result<String, CacheTestErr> {
        for i in 0..N {
            self.ans[i] = i as u32;
        }
        let start = CACHE_SIZE / 4;
        for i in 0..N {
            self.ans[start + i] = (start + i) as  u32;
        }
        for i in 0..N {
            xs_assert_eq!(self.ans[i], i as u32, self.err_type());
            xs_assert_eq!(self.ans[start + i], (start + i) as u32, self.err_type());
        }
        Ok(String::from("access_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CacheTestErr> {
        for _ in 0..bench_size {
            for i in 0..N {
                self.ans[i] = i as u32;
            }
            let start = CACHE_SIZE / 4;
            for i in 0..N {
                self.ans[start + i] = (start + i) as  u32;
            }
            for i in 0..N {
                xs_assert_eq!(self.ans[i], i as u32, self.err_type());
                xs_assert_eq!(self.ans[start + i], (start + i) as u32, self.err_type());
            }
        }
        Ok(String::from("access_bench_test"))
    }

}