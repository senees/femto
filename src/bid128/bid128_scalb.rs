use crate::bid128::bid_internal::*;
use crate::BID128;

const MAX_DECIMAL_EXPONENT_128: u32 = 12287;

impl BID128 {
  ///
  pub fn scalbn(&self, n: i32) -> Self {
    let mut sign = 0_u64;
    let mut exponent = 0_i32;
    let mut coefficient = BID128::default();
    if !unpack_bid128_value(&mut sign, &mut exponent, &mut coefficient, self) {
      todo!();
    }
    let exp64 = exponent as i64 + n as i64;
    exponent = exp64 as i32;
    if exponent as u32 <= MAX_DECIMAL_EXPONENT_128 {
      let mut res = BID128::default();
      bid_get_bid128_very_fast(&mut res, sign, exponent, coefficient);
      return res;
    }
    todo!();
  }
}
