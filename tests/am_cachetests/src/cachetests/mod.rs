/// nexus-am cachetests

pub mod access;

use benchmark::{CacheTestErr, BenchMark};
use alloc::{
    vec,
    vec::Vec,
    string::String,
};

const BENCH_SIZE: usize = 20;

pub fn test_all() -> Vec<Result<String, CacheTestErr>> {
    let mut access_test = access::AccessTest::new();
    let results = vec![
        access_test.single_test(),
        access_test.bench_test(BENCH_SIZE),
    ];
    results
}
