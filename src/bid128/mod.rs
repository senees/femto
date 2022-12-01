//! TBD

use crate::bid128::bid_functions::ExceptionStatusFlag;
use std::fmt;

mod bid128_from_int;
mod bid128_from_string;
mod bid128_scalb;
mod bid128_to_string;
mod bid128_to_string_macros;
mod bid128_to_string_tables;
mod bid_decimal_data;
mod bid_functions;
mod bid_internal;
#[cfg(test)]
mod tests;

/// 128-bit decimal number.
#[derive(Default, Copy, Clone)]
pub struct BID128 {
  /// Two 64-bit fields holding encoded decimal value.
  w: [u64; 2],
  pub flags: ExceptionStatusFlag,
}

impl fmt::Debug for BID128 {
  /// Converts [BID128] into debug string.
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:x},{:x}]", self.w[0], self.w[1])
  }
}

/// Internal macro for creating 128-bit decimal value.
macro_rules! __bid128 {
  ($w0:expr, $w1:expr) => {
    BID128 { w: [$w0, $w1], flags: 0 }
  };
}

pub(crate) use __bid128;
