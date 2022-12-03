//! Implementation of 128-bit functions.

use crate::bid128::BID128;

///
pub fn bid128_quantize(x: &BID128, y: &BID128) -> BID128 {
  x.quantize(y)
}
