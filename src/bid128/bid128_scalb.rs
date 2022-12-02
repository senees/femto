use super::__bid128;
use crate::bid128::bid_functions::*;
use crate::bid128::bid_internal::*;
use crate::BID128;

impl BID128 {
  ///
  pub fn scalbn(&self, n: i32) -> Self {
    let mut sign_x = 0_u64;
    let mut exponent_x = 0_i32;
    let mut cx = __bid128!();
    let mut res = __bid128!();
    if !unpack_bid128_value(&mut sign_x, &mut exponent_x, &mut cx, self) {
      if (self.w[1] & SNAN_MASK64) == SNAN_MASK64 {
        res.flags |= BID_INVALID_EXCEPTION;
      }
      res.w[1] = cx.w[1] & QUIET_MASK64;
      res.w[0] = cx.w[0];
      if !cx.w[1] > 0 {
        let mut exp64 = exponent_x as i64 + n as i64;
        if exp64 < 0 {
          exp64 = 0
        };
        if exp64 > DECIMAL_MAX_EXPON_128 as i64 {
          exp64 = DECIMAL_MAX_EXPON_128 as i64;
        }
        bid_get_bid128_very_fast(&mut res, sign_x, exp64 as i32, cx);
      }
      return res;
    }
    let mut exp64 = exponent_x as i64 + n as i64;
    exponent_x = exp64 as i32;
    // fast return when no overflow
    if (exponent_x as u32) <= DECIMAL_MAX_EXPON_128 {
      bid_get_bid128_very_fast(&mut res, sign_x, exponent_x, cx);
      return res;
    }
    // check for overflow
    if exp64 > DECIMAL_MAX_EXPON_128 as i64 {
      if cx.w[1] < 0x314DC6448D93_u64 {
        // try to normalize coefficient
        loop {
          let cbid_x8 = __bid128!(cx.w[0] << 3, (cx.w[1] << 3) | (cx.w[0] >> 61));
          let cx2 = __bid128!(cx.w[0] << 1, (cx.w[1] << 1) | (cx.w[0] >> 63));
          __add_128_128!(cx, cx2, cbid_x8);
          exponent_x -= 1;
          exp64 = -1;
          if !(cx.w[1] < 0x314DC6448D93_u64 && exp64 > DECIMAL_MAX_EXPON_128 as i64) {
            break;
          }
        }
      }
      if exp64 <= DECIMAL_MAX_EXPON_128 as i64 {
        bid_get_bid128_very_fast(&mut res, sign_x, exponent_x, cx);
        return res;
      } else {
        exponent_x = 0x7fffffff; // still overflow
      }
    }
    bid_get_bid128(&mut res, sign_x, exponent_x, cx, self.rounding);
    res
  }
}
