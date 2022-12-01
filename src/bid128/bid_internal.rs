use crate::bid128::bid_decimal_data::*;
use crate::BID128;

pub const SPECIAL_ENCODING_MASK64: u64 = 0x6000000000000000;
pub const INFINITY_MASK64: u64 = 0x7800000000000000;
pub const SINFINITY_MASK64: u64 = 0xF800000000000000;
pub const EXPONENT_MASK128: i32 = 0x3FFF;
pub const NAN_MASK64: u64 = 0x7c00000000000000;
pub const SNAN_MASK64: u64 = 0x7e00000000000000;
pub const QUIET_MASK64: u64 = 0xfdffffffffffffff;
pub const SMALL_COEFF_MASK128: u64 = 0x0001FFFFFFFFFFFF;

///
macro_rules! unsigned_compare_ge_128 {
  ($a:expr, $b:expr) => {{
    (($a.w[1] > $b.w[1]) || (($a.w[1] == $b.w[1]) && ($a.w[0] >= $b.w[0])))
  }};
}

/// Add 128-bit value to 128-bit value, assume no carry-out.
macro_rules! __add_128_128 {
  ($r128:expr, $a128:expr, $b128:expr) => {{
    let mut q128 = BID128 {
      w: [$b128.w[0] + $a128.w[0], $a128.w[1] + $b128.w[1]],
      flags: 0,
    };
    if q128.w[0] < $b128.w[0] {
      q128.w[1] += 1;
    }
    $r128.w[0] = q128.w[0];
    $r128.w[1] = q128.w[1];
  }};
}

pub(crate) use __add_128_128;

#[inline(always)]
pub fn unpack_bid128_value(sign: &mut u64, exponent: &mut i32, coefficient: &mut BID128, x: &BID128) -> bool {
  // check the sign
  *sign = (x.w[1]) & 0x8000000000000000_u64;
  // special encodings
  if (x.w[1] & INFINITY_MASK64) >= SPECIAL_ENCODING_MASK64 {
    if (x.w[1] & INFINITY_MASK64) < INFINITY_MASK64 {
      // non-canonical input
      coefficient.w[0] = 0;
      coefficient.w[1] = 0;
      *exponent = (((x.w[1]) >> 47) as i32) & EXPONENT_MASK128;
      return false;
    }
    // 10^33
    let t33 = BID_POWER10_TABLE_128[33];
    coefficient.w[0] = x.w[0];
    coefficient.w[1] = x.w[1] & 0x00003FFFFFFFFFFF_u64;
    if unsigned_compare_ge_128!(coefficient, t33) {
      // non-canonical
      coefficient.w[1] = (x.w[1]) & 0xfe00000000000000_u64;
      coefficient.w[0] = 0;
    } else {
      coefficient.w[1] = (x.w[1]) & 0x00003FFFFFFFFFFF_u64;
    }
    if (x.w[1] & NAN_MASK64) == INFINITY_MASK64 {
      coefficient.w[0] = 0;
      coefficient.w[1] = x.w[1] & SINFINITY_MASK64;
    }
    *exponent = 0;
    return false; // NaN or Infinity
  }
  let mut coeff = BID128 {
    w: [x.w[0], x.w[1] & SMALL_COEFF_MASK128],
    ..Default::default()
  };
  // 10^34
  let t34 = BID_POWER10_TABLE_128[34];
  // check for non-canonical values
  if unsigned_compare_ge_128!(coeff, t34) {
    coeff.w[0] = 0;
    coeff.w[1] = 0;
  }
  coefficient.w[0] = coeff.w[0];
  coefficient.w[1] = coeff.w[1];
  let e = (x.w[1]) >> 49;
  *exponent = (e as i32) & EXPONENT_MASK128;
  coeff.w[0] | coeff.w[1] != 0
}

pub fn bid_get_bid128_very_fast(res: &mut BID128, sign: u64, exponent: i32, coefficient: BID128) -> &BID128 {
  res.w[0] = coefficient.w[0];
  let tmp = (exponent as u64) << 49;
  res.w[1] = sign | tmp | coefficient.w[1];
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unpack_bid128_value_0001() {
    let x = BID128::from(12_u64);
    let mut coefficient = BID128::from(0_i32);
    let mut sign = 0_u64;
    let mut exponent = 0_i32;
    assert!(unpack_bid128_value(&mut sign, &mut exponent, &mut coefficient, &x));
    assert_eq!(0, sign);
    assert_eq!(6176, exponent);
    assert_eq!("+12E-6176", format!("{}", coefficient.to_string()))
  }

  #[test]
  fn test_unpack_bid128_value_0002() {
    let x = BID128::from(-12_i64);
    let mut coefficient = BID128::from(0_i32);
    let mut sign = 0_u64;
    let mut exponent = 0_i32;
    assert!(unpack_bid128_value(&mut sign, &mut exponent, &mut coefficient, &x));
    assert_eq!(0x8000000000000000, sign);
    assert_eq!(6176, exponent);
    assert_eq!("+12E-6176", format!("{}", coefficient.to_string()))
  }
}
