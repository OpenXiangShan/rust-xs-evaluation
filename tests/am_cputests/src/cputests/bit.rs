/// BitTest Implementation

use crate::benchmark::BenchMark;
use crate::bit::BitIndex;

#[no_mangle]
pub struct BitTest {
    bit_val: usize
}

impl BenchMark for BitTest {
    fn new() -> Self {
        let bit_val = 0usize;
        Self {
            bit_val
        }
    }

    fn test(&mut self) {
        for i in 0..usize::bit_length() {
            assert_eq!(self.bit_val.bit(i), false);
            self.bit_val.set_bit(i, i % 2 == 0);
        }
        assert_eq!(self.bit_val, 0x5555_5555_5555_5555);
        assert_eq!(self.bit_val.bit_range(0..3), 0x05);
        assert_eq!(self.bit_val.bit_range(4..7), 0x05);
        assert_eq!(self.bit_val.bit_range(8..11), 0x05);
        assert_eq!(self.bit_val.bit_range(12..15), 0x05);
        assert_eq!(self.bit_val.bit_range(16..19), 0x05);
        assert_eq!(self.bit_val.bit_range(20..23), 0x05);
        assert_eq!(self.bit_val.bit_range(24..27), 0x05);
        assert_eq!(self.bit_val.bit_range(28..31), 0x05);
        assert_eq!(self.bit_val.bit_range(32..35), 0x05);
        assert_eq!(self.bit_val.bit_range(36..39), 0x05);
        assert_eq!(self.bit_val.bit_range(40..43), 0x05);
        assert_eq!(self.bit_val.bit_range(44..47), 0x05);
        assert_eq!(self.bit_val.bit_range(48..51), 0x05);
        assert_eq!(self.bit_val.bit_range(52..55), 0x05);
        assert_eq!(self.bit_val.bit_range(56..59), 0x05);
        assert_eq!(self.bit_val.bit_range(60..63), 0x05);
    }
}