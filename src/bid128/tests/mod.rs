//! Utilities for testing.

use crate::bid128::__bid128;
use crate::BID128;

mod bid128_from_int32;
mod bid128_from_int64;
mod bid128_scalbn;

#[cfg(target_endian = "big")]
const BID_HIGH_128W: usize = 0;
#[cfg(target_endian = "big")]
const BID_LOW_128W: usize = 1;

#[cfg(target_endian = "little")]
const BID_HIGH_128W: usize = 1;
#[cfg(target_endian = "little")]
const BID_LOW_128W: usize = 0;

const MASK_STEERING_BITS: u64 = 0x6000000000000000;
const SMALL_COEFF_MASK128: u64 = 0x0001ffffffffffff;
const MASK_SIG2: u64 = 0x00007fffffffffff;
const EXP_P1: u64 = 0x0002000000000000;

fn get_mant_128(x: &BID128) -> BID128 {
  let mut result = __bid128!();
  if (x.w[BID_HIGH_128W] & MASK_STEERING_BITS) == MASK_STEERING_BITS {
    result.w[BID_HIGH_128W] = (x.w[BID_HIGH_128W] & MASK_SIG2) | EXP_P1;
    result.w[BID_LOW_128W] = x.w[BID_LOW_128W];
  } else {
    result.w[BID_HIGH_128W] = x.w[BID_HIGH_128W] & SMALL_COEFF_MASK128;
    result.w[BID_LOW_128W] = x.w[BID_LOW_128W];
  }
  result
}
