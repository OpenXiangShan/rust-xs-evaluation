/// BitTest Implementation

use crate::benchmark::{BenchMark, CpuTestErr, xs_assert_eq};
use crate::bit::BitIndex;
use crate::alloc::string::String;

#[no_mangle]
pub struct BitTest {
    bit_val: usize
}

impl BenchMark<CpuTestErr> for BitTest {
    fn new() -> Self {
        let bit_val = 0usize;
        Self {
            bit_val
        }
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        for i in 0..usize::bit_length() {
            xs_assert_eq!(self.bit_val.bit(i), false, self.err_type());
            self.bit_val.set_bit(i, i % 2 == 0);
        }
        xs_assert_eq!(self.bit_val, 0x5555_5555_5555_5555, self.err_type());
        for i in 0..16 {
            xs_assert_eq!(self.bit_val.bit_range((i << 2)..(i << 2) + 3), 0x05, self.err_type());
        }
        // xs_assert_eq!(self.bit_val.bit_range(0..3), 0x05, self.err_type());
        Ok(String::from("bit_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        for _ in 0..bench_size {
            self.bit_val = 0x0;
            for i in 0..usize::bit_length() {
                xs_assert_eq!(self.bit_val.bit(i), false, self.err_type());
                self.bit_val.set_bit(i, i % 2 == 0);
            }
            xs_assert_eq!(self.bit_val, 0x5555_5555_5555_5555, self.err_type());
            for i in 0..16 {
                xs_assert_eq!(self.bit_val.bit_range((i << 2)..(i << 2) + 3), 0x05, self.err_type());
            }
        }
        Ok(String::from("bit_bench_test"))
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::BitTestErr
    }
}