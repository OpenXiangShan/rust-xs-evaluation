/// nexus-am cputests


pub mod add;
pub mod bit;
pub mod add_u64;
pub mod bubble_sort;
pub mod mul_u64;
pub mod div;

use benchmark::{CpuTestErr, BenchMark};
use alloc::{
    vec,
    vec::Vec,
    string::String,
};

const BENCH_SIZE: usize = 20;

pub fn test_all() -> Vec<Result<String, CpuTestErr>> {
    let mut add_test = add::AddTest::new();
    let mut bit_test = bit::BitTest::new();
    let mut addu64_test = add_u64::AddU64Test::new();
    let mut bubble_sort_test = bubble_sort::BubbleSortTest::new();
    let mut mulu64_test = mul_u64::MulU64Test::new();
    let mut div_test = div::DivTest::new();
    let results = vec![
        add_test.single_test(),
        add_test.bench_test(BENCH_SIZE),
        bit_test.single_test(),
        bit_test.bench_test(BENCH_SIZE),
        addu64_test.single_test(),
        addu64_test.bench_test(BENCH_SIZE),
        bubble_sort_test.single_test(),
        bubble_sort_test.bench_test(BENCH_SIZE),
        mulu64_test.single_test(),
        mulu64_test.bench_test(BENCH_SIZE),
        div_test.single_test(),
        div_test.bench_test(BENCH_SIZE),
    ];
    results
}