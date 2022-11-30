//! TBD

use std::fmt;

mod bid128_from_int;
mod bid128_from_string;
mod bid128_scalb;
mod bid128_to_string;
mod bid128_to_string_macros;
mod bid128_to_string_tables;
mod bid_decimal_data;
mod bid_internal;

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
