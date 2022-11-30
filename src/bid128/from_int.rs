//!

use super::*;

impl From<u64> for BID128 {
  /// Converts [u64] into [BID128].
  fn from(x: u64) -> Self {
    let mut res = BID128::default();
    res.w[1] = 0x3040000000000000_u64;
    res.w[0] = x;
    res
  }
}

impl From<i64> for BID128 {
  /// Converts [i64] into [BID128].
  fn from(x: i64) -> Self {
    let mut res = BID128::default();
    if (x as u64) & 0x8000000000000000_u64 == 0x8000000000000000_u64 {
      res.w[1] = 0xB040000000000000_u64;
      res.w[0] = !(x as u64) + 1_u64;
    } else {
      res.w[1] = 0x3040000000000000_u64;
      res.w[0] = x as u64;
    }
    res
  }
}
