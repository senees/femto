//!

use super::*;

impl From<u32> for BID128 {
  /// Converts [u32] into [BID128].
  fn from(x: u32) -> Self {
    __bid128!(x as u64, 0x3040000000000000_u64)
  }
}

impl From<i32> for BID128 {
  /// Converts [i32] into [BID128].
  fn from(x: i32) -> Self {
    if (x as u32) & 0x80000000_u32 == 0x80000000_u32 {
      __bid128!((!(x as u32) + 1_u32) as u64, 0xB040000000000000_u64)
    } else {
      __bid128!(x as u64, 0x3040000000000000_u64)
    }
  }
}

impl From<u64> for BID128 {
  /// Converts [u64] into [BID128].
  fn from(x: u64) -> Self {
    __bid128!(x, 0x3040000000000000_u64)
  }
}

impl From<i64> for BID128 {
  /// Converts [i64] into [BID128].
  fn from(x: i64) -> Self {
    if (x as u64) & 0x8000000000000000_u64 == 0x8000000000000000_u64 {
      __bid128!(!(x as u64) + 1_u64, 0xB040000000000000_u64)
    } else {
      __bid128!(x as u64, 0x3040000000000000_u64)
    }
  }
}
