//! TBD

use std::fmt;

mod from_int;

/// 128-bit decimal number.
#[derive(Default, Copy, Clone)]
pub struct BID128 {
  w: [u64; 2],
}

impl fmt::Debug for BID128 {
  /// Converts [BID128] into debug string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:X},{:X}]", self.w[0], self.w[1])
  }
}
