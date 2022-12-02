use crate::bid128::__bid128;
use crate::bid128::bid_decimal_data::*;
use crate::bid128::bid_functions::*;
use crate::BID128;

pub const SPECIAL_ENCODING_MASK64: u64 = 0x6000000000000000;
pub const INFINITY_MASK64: u64 = 0x7800000000000000;
pub const SINFINITY_MASK64: u64 = 0xF800000000000000;
pub const EXPONENT_MASK128: i32 = 0x3FFF;
pub const NAN_MASK64: u64 = 0x7c00000000000000;
pub const SNAN_MASK64: u64 = 0x7e00000000000000;
pub const QUIET_MASK64: u64 = 0xfdffffffffffffff;
pub const SMALL_COEFF_MASK128: u64 = 0x0001FFFFFFFFFFFF;

// BID128 definitions
pub const DECIMAL_MAX_EXPON_128: u32 = 12287;
pub const MAX_FORMAT_DIGITS_128: u32 = 34;

/*******************************************************************************
 *                                                                             *
 * Logical shift macros,                                                       *
 *                                                                             *
 *******************************************************************************/

/// Shift right 128-bit value.
macro_rules! __shr_128 {
  ($q:expr, $a:expr, $k:expr) => {{
    $q.w[0] = $a.w[0] >> $k;
    $q.w[0] |= $a.w[1] << (64 - $k);
    $q.w[1] = $a.w[1] >> $k;
  }};
}

/// Shift right (long) 128-bit value.
macro_rules! __shr_128_long {
  ($q:expr, $a:expr, $k:expr) => {{
    if $k < 64 {
      $q.w[0] = $a.w[0] >> $k;
      $q.w[0] |= $a.w[1] << (64 - $k);
      $q.w[1] = $a.w[1] >> $k;
    } else {
      $q.w[0] = $a.w[1] >> ($k - 64);
      $q.w[1] = 0;
    }
  }};
}

/// Shift left 128-bit value.
macro_rules! __shl_128_long {
  ($q:expr, $a:expr, $k:expr) => {{
    if ($k < 64) {
      $q.w[1] = $a.w[1] << $k;
      $q.w[1] |= $a.w[0] >> (64 - $k);
      $q.w[0] = $a.w[0] << $k;
    } else {
      $q.w[1] = $a.w[0] << ($k - 64);
      $q.w[0] = 0;
    }
  }};
}

/*******************************************************************************
 *                                                                             *
 * Compare macros.                                                             *
 *                                                                             *
 *******************************************************************************/

///
macro_rules! __unsigned_compare_ge_128 {
  ($a:expr, $b:expr) => {{
    (($a.w[1] > $b.w[1]) || (($a.w[1] == $b.w[1]) && ($a.w[0] >= $b.w[0])))
  }};
}

/*******************************************************************************
 *                                                                             *
 * Add/subtract macros.                                                        *
 *                                                                             *
 *******************************************************************************/

/// Add 64-bit value to 128-bit.
macro_rules! __add_128_64 {
  ($r128:expr, $a128:expr, $b64:expr) => {{
    let mut r64h = $a128.w[1];
    $r128.w[0] = $b64 + $a128.w[0];
    if $r128.w[0] < $b64 {
      r64h += 1;
    }
    $r128.w[1] = r64h;
  }};
}

/// Add 128-bit value to 128-bit value, assume no carry-out.
macro_rules! __add_128_128 {
  ($r128:expr, $a128:expr, $b128:expr) => {{
    let mut q128 = __bid128!($b128.w[0].wrapping_add($a128.w[0]), $a128.w[1].wrapping_add($b128.w[1]));
    if q128.w[0] < $b128.w[0] {
      q128.w[1] += 1;
    }
    $r128.w[0] = q128.w[0];
    $r128.w[1] = q128.w[1];
  }};
}
pub(crate) use __add_128_128;

/*******************************************************************************
 *                                                                             *
 * Unsigned multiply macros.                                                   *
 *                                                                             *
 *******************************************************************************/

/// Get full 64x64bit product.
macro_rules! __mul_64x64_to_128 {
  ($p:expr, $cx:expr, $cy:expr) => {{
    let cxh: u64 = $cx >> 32;
    let cxl: u64 = ($cx as u32) as u64;
    let cyh: u64 = $cy >> 32;
    let cyl: u64 = ($cy as u32) as u64;
    let mut pm = cxh * cyl;
    let mut ph = cxh * cyh;
    let pl = cxl * cyl;
    let pm2 = cxl * cyh;
    ph += (pm >> 32);
    pm = ((pm as u32) as u64) + pm2 + (pl >> 32);
    $p.w[1] = ph + (pm >> 32);
    $p.w[0] = (pm << 32) + (pl as u32) as u64;
  }};
}

macro_rules! __mul_128x128_full {
  ($qh:expr, $ql:expr, $a:expr, $b:expr) => {{
    let mut albh = __bid128!();
    let mut ahbl = __bid128!();
    let mut albl = __bid128!();
    let mut ahbh = __bid128!();
    __mul_64x64_to_128!(albh, $a.w[0], $b.w[1]);
    __mul_64x64_to_128!(ahbl, $b.w[0], $a.w[1]);
    __mul_64x64_to_128!(albl, $a.w[0], $b.w[0]);
    __mul_64x64_to_128!(ahbh, $a.w[1], $b.w[1]);
    let mut qm = __bid128!();
    __add_128_128!(qm, albh, ahbl);
    $ql.w[0] = albl.w[0];
    let mut qm2 = __bid128!();
    __add_128_64!(qm2, qm, albl.w[1]);
    __add_128_64!($qh, ahbh, qm2.w[1]);
    $ql.w[1] = qm2.w[0];
  }};
}

macro_rules! __add_carry_out {
  ($s:expr, $cy:expr, $x:expr, $y:expr) => {{
    let x1: u64 = $x;
    $s = $x + $y;
    $cy = if ($s < x1) { 1 } else { 0 };
  }};
}

macro_rules! __add_carry_in_out {
  ($s:expr, $cy:expr, $x:expr, $y:expr, $ci: expr) => {{
    let x1: u64 = $x + $ci;
    $s = x1 + $y;
    $cy = if ($s < x1 || x1 < $ci) { 1 } else { 0 };
  }};
}

/*

#define __add_carry_in_out(S, CY, X, Y, CI)    \
{                                             \
BID_UINT64 X1;                                    \
  X1 = X + CI;                              \
  S = X1 + Y;                               \
  CY = ((S<X1) || (X1<CI)) ? 1 : 0;          \
}

 */

macro_rules! __set_status_flags {
  ($flags:expr, $status:expr) => {
    $flags |= $status
  };
}

macro_rules! __is_inexact {
  ($flags:expr) => {{
    $flags & BID_INEXACT_EXCEPTION > 0
  }};
}

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
    if __unsigned_compare_ge_128!(coefficient, t33) {
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
  let mut coeff = __bid128!(x.w[0], x.w[1] & SMALL_COEFF_MASK128);
  // 10^34
  let t34 = BID_POWER10_TABLE_128[34];
  // check for non-canonical values
  if __unsigned_compare_ge_128!(coeff, t34) {
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

pub fn handle_uf_128(pres: &mut BID128, sgn: u64, expon: i32, mut cq: BID128, prounding_mode: Rounding) -> &BID128 {
  // underflow occurs
  if (expon + MAX_FORMAT_DIGITS_128 as i32) < 0 {
    pres.w[1] = sgn;
    pres.w[0] = 0;
    pres.flags |= BID_UNDERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION;
    if (sgn > 0 && prounding_mode == BID_ROUNDING_DOWN) || (sgn > 0 && prounding_mode == BID_ROUNDING_UP) {
      pres.w[0] = 1u64;
    }
    return pres;
  }
  let ed2 = (0 - expon) as usize;
  // add rounding constant to `cq`
  let mut rmode = prounding_mode;
  if sgn > 0 && ((rmode - 1) as u32) < 2 {
    rmode = 3 - rmode
  };

  let t128 = BID_ROUND_CONST_TABLE_128[rmode as usize][ed2];
  let mut carry;
  __add_carry_out!(cq.w[0], carry, t128.w[0], cq.w[0]);
  cq.w[1] = cq.w[1] + t128.w[1] + carry;
  let tp128 = BID_RECIPROCALS10_128[ed2];
  let mut qh = __bid128!();
  let mut ql = __bid128!();
  __mul_128x128_full!(qh, ql, cq, tp128);
  let amount = BID_RECIP_SCALE[ed2];
  if amount >= 64 {
    cq.w[0] = qh.w[1] >> (amount - 64);
    cq.w[1] = 0;
  } else {
    __shr_128!(cq, qh, amount);
  }

  if prounding_mode > 0 {
    if cq.w[0] & 1 > 0 {
      // check whether fractional part of initial_P/10^ed1 is exactly .5

      // get remainder
      let mut qh1 = __bid128!();
      __shl_128_long!(qh1, qh, (128 - amount));

      if qh1.w[1] == 0 && qh1.w[0] == 0 && (ql.w[1] < BID_RECIPROCALS10_128[ed2].w[1] || (ql.w[1] == BID_RECIPROCALS10_128[ed2].w[1] && ql.w[0] < BID_RECIPROCALS10_128[ed2].w[0]))
      {
        cq.w[0] -= 1;
      }
    }
  }

  if __is_inexact!(pres.flags) {
    __set_status_flags!(pres.flags, BID_UNDERFLOW_EXCEPTION);
  } else {
    let mut status = BID_INEXACT_EXCEPTION;
    // get remainder
    let mut qh1 = __bid128!();
    __shl_128_long!(qh1, qh, (128 - amount));
    match rmode {
      BID_ROUNDING_TO_NEAREST | BID_ROUNDING_TIES_AWAY => {
        // test whether fractional part is 0
        if (qh1.w[1] == 0x8000000000000000u64 && qh1.w[0] == 0)
          && (ql.w[1] < BID_RECIPROCALS10_128[ed2].w[1] || (ql.w[1] == BID_RECIPROCALS10_128[ed2].w[1] && ql.w[0] < BID_RECIPROCALS10_128[ed2].w[0]))
        {
          status = BID_EXACT_STATUS;
        }
      }
      BID_ROUNDING_DOWN | BID_ROUNDING_TO_ZERO => {
        if qh1.w[1] == 0
          && qh1.w[0] == 0
          && (ql.w[1] < BID_RECIPROCALS10_128[ed2].w[1] || (ql.w[1] == BID_RECIPROCALS10_128[ed2].w[1] && ql.w[0] < BID_RECIPROCALS10_128[ed2].w[0]))
        {
          status = BID_EXACT_STATUS;
        }
      }
      _ => {
        // round up
        let mut stemp = __bid128!();
        let cy;
        __add_carry_out!(stemp.w[0], cy, ql.w[0], BID_RECIPROCALS10_128[ed2].w[0]);
        __add_carry_in_out!(stemp.w[1], carry, ql.w[1], BID_RECIPROCALS10_128[ed2].w[1], cy);
        __shr_128_long!(qh, qh1, (128 - amount));
        let tmp = __bid128!(1, 0);
        let mut tmp1 = __bid128!();
        __shl_128_long!(tmp1, tmp, amount);
        qh.w[0] += carry;
        if qh.w[0] < carry {
          qh.w[1] += 1;
        }
        if __unsigned_compare_ge_128!(qh, tmp1) {
          status = BID_EXACT_STATUS;
        }
      }
    }
    if status != BID_EXACT_STATUS {
      __set_status_flags!(pres.flags, BID_UNDERFLOW_EXCEPTION | status);
    }
  }
  pres.w[1] = sgn | cq.w[1];
  pres.w[0] = cq.w[0];
  pres
}

pub fn bid_get_bid128(pres: &mut BID128, sgn: u64, mut expon: i32, mut coeff: BID128, prounding_mode: Rounding) -> &BID128 {
  // coeff==10^34?
  if coeff.w[1] == 0x0001ed09bead87c0u64 && coeff.w[0] == 0x378d8e6400000000u64 {
    expon += 1;
    // set coefficient to 10^33
    coeff.w[1] = 0x0000314dc6448d93u64;
    coeff.w[0] = 0x38c15b0a00000000u64;
  }
  // check overflow, underflow
  if expon < 0 || expon > DECIMAL_MAX_EXPON_128 as i32 {
    // check underflow
    if expon < 0 {
      return handle_uf_128(pres, sgn, expon, coeff, prounding_mode);
    }
  }

  /*


    if (expon - MAX_FORMAT_DIGITS_128 <= DECIMAL_MAX_EXPON_128) {
      T = bid_power10_table_128[MAX_FORMAT_DIGITS_128 - 1];
      while (__unsigned_compare_gt_128 (T, coeff) && expon > DECIMAL_MAX_EXPON_128) {
        coeff.w[1] = (coeff.w[1] << 3) + (coeff.w[1] << 1) + (coeff.w[0] >> 61) + (coeff.w[0] >> 63);
        tmp2 = coeff.w[0] << 3;
        coeff.w[0] = (coeff.w[0] << 1) + tmp2;
        if (coeff.w[0] < tmp2) coeff.w[1]++;
        expon--;
      }
    }
    if (expon > DECIMAL_MAX_EXPON_128) {
      if (!(coeff.w[1] | coeff.w[0])) {
        pres->w[1] = sgn | (((BID_UINT64) DECIMAL_MAX_EXPON_128) << 49);
        pres->w[0] = 0;
        return pres;
      }
      // OF
      #ifdef BID_SET_STATUS_FLAGS
      __set_status_flags (fpsc, BID_OVERFLOW_EXCEPTION | BID_INEXACT_EXCEPTION);
      #endif
      #ifndef IEEE_ROUND_NEAREST_TIES_AWAY
      #ifndef IEEE_ROUND_NEAREST
      if (*prounding_mode == BID_ROUNDING_TO_ZERO || (sgn && *prounding_mode == BID_ROUNDING_UP) || (!sgn && *prounding_mode == BID_ROUNDING_DOWN)) {
        pres->w[1] = sgn | LARGEST_BID128_HIGH;
        pres->w[0] = LARGEST_BID128_LOW;
      } else
      #endif
      #endif
      {
        pres->w[1] = sgn | INFINITY_MASK64;
        pres->w[0] = 0;
      }
      return pres;
    }
  }
  pres->w[0] = coeff.w[0];
  tmp = expon;
  tmp <<= 49;
  pres->w[1] = sgn | tmp | coeff.w[1];

     */
  //
  pres
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
