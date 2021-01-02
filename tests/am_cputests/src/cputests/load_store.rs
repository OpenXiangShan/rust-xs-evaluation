/// LoadStoreTest Implementation
/// 

use crate::benchmark::*;
#[allow(unused_imports)]
use crate::println;
use alloc::{
    string::String,
};

#[no_mangle]
#[repr(C)]
pub struct LoadStoreTest {
    mem: [u16; 8],
    lh_ans: [i32; 8],
    lhu_ans: [u32; 8],
    sh_ans: [u32; 8],
    lwlr_ans: [u32; 4],
}

#[allow(overflowing_literals)]
impl BenchMark for LoadStoreTest {
    fn new() -> Self {
        let mem = [0x0, 0x0258, 0x4abc, 0x7fff, 0x8000, 0x8100, 0xabcd, 0xffff];
        let lh_ans = [0x00000000, 0x00000258, 0x00004abc, 0x00007fff, 0xffff8000, 0xffff8100, 0xffffabcd, 0xffffffff];
        let lhu_ans = [0x00000000, 0x00000258, 0x00004abc, 0x00007fff, 0x00008000, 0x00008100, 0x0000abcd, 0x0000ffff];
        let sh_ans = [0x0000fffd, 0x0000fff7, 0x0000ffdf, 0x0000ff7f, 0x0000fdff, 0x0000f7ff, 0x0000dfff, 0x00007fff];
        let lwlr_ans = [0xbc025800, 0x7fff4a, 0xcd810080, 0xffffab];
        Self {
            mem,
            lh_ans,
            lhu_ans,
            sh_ans,
            lwlr_ans,
        }
    }

    fn err_type(&self) -> CpuTestErr {
        CpuTestErr::LoadStoreTestErr
    }

    fn single_test(&mut self) -> Result<String, CpuTestErr> {
        xs_assert_eq!(self.mem.len(), self.lh_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len(), self.lhu_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len(), self.sh_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len() / 2, self.lwlr_ans.len(), self.err_type());
        for i in 0..self.mem.len() {
            xs_assert_eq!(self.mem[i] as i32, self.lh_ans[i], self.err_type());
        }
        for i in 0..self.mem.len() {
            xs_assert_eq!(self.mem[i] as u32, self.lhu_ans[i], self.err_type());
        }
        for i in 0..self.mem.len() / 2 - 1 {
            let x = unsafe { *((&self.mem as *const u16 as usize + 0x2 + 0x4 * i) as *const u32) };
            xs_assert_eq!(x, self.lwlr_ans[i], self.err_type());
        }
        for i in 0..self.mem.len() {
            self.mem[i] = !(1 << (2 * i + 1));
            xs_assert_eq!(self.mem[i] as u32, self.sh_ans[i], self.err_type());
        }
        Ok(String::from("load_store_single_test"))
    }

    fn bench_test(&mut self, bench_size: usize) -> Result<String, CpuTestErr> {
        xs_assert_eq!(self.mem.len(), self.lh_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len(), self.lhu_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len(), self.sh_ans.len(), self.err_type());
        xs_assert_eq!(self.mem.len() / 2, self.lwlr_ans.len(), self.err_type());
        for _ in 0..bench_size {
            for i in 0..self.mem.len() {
                xs_assert_eq!(self.mem[i] as i32, self.lh_ans[i], self.err_type());
            }
            for i in 0..self.mem.len() {
                xs_assert_eq!(self.mem[i] as u32, self.lhu_ans[i], self.err_type());
            }
            for i in 0..self.mem.len() / 2 - 1 {
                let x = unsafe { *((&self.mem as *const u16 as usize + 0x2 + 0x4 * i) as *const u32) };
                xs_assert_eq!(x, self.lwlr_ans[i], self.err_type());
            }
        }
        for i in 0..self.mem.len() {
            self.mem[i] = !(1 << (2 * i + 1));
            xs_assert_eq!(self.mem[i] as u32, self.sh_ans[i], self.err_type());
        }
        Ok(String::from("load_store_bench_test"))
    }
}