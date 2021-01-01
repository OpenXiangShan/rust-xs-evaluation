/// nexus-am cputests


pub mod add;
pub mod bit;
pub mod add_u64;

use benchmark::{CpuTestErr, BenchMark};
use alloc::{
    vec::Vec,
    string::String,
};

const BENCH_SIZE: usize = 20;

pub fn test_all() -> Vec<Result<String, CpuTestErr>> {
    let mut results = Vec::new();
    let mut add_test = add::AddTest::new();
    let mut bit_test = bit::BitTest::new();
    let mut addu64_test = add_u64::AddU64Test::new();
    results.push(add_test.single_test());
    results.push(add_test.bench_test(BENCH_SIZE));
    results.push(bit_test.single_test());
    results.push(bit_test.bench_test(BENCH_SIZE));
    results.push(addu64_test.single_test());
    results.push(addu64_test.bench_test(BENCH_SIZE));
    results
}