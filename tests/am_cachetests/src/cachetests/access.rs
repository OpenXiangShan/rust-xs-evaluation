/// AccessTest Implementation
/// 

use crate::benchmark::{BenchMark, CacheTestErr, xs_assert_eq};
use crate::alloc::{
    vec::Vec,
    string::String,
};

const N: usize = 64;
const CACHE_SIZE: usize = 32 * 1024;

#[no_mangle]
pub struct AccessTest {
    data: Vec<u32>
}

