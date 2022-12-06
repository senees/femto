use crate::bid128::bid_internal::SIGN_MASK64;
use crate::bid128::{__bid128, BID128};

const P_INF: BID128 = __bid128!(0, 0x7800000000000000u64);
const N_INF: BID128 = __bid128!(0, 0xf800000000000000u64);
const NAN: BID128 = __bid128!(0, 0x7c00000000000000u64);
const SNAN: BID128 = __bid128!(0, 0x7e00000000000000u64);

const PREFIXES: [&str; 10] = ["INFINITY", "+INFINITY", "-INFINITY", "INF", "+INF", "-INF", "NAN", "+NAN", "SNAN", "+SNAN"];
const VALUES: [BID128; 10] = [NAN, NAN, NAN, NAN, NAN, NAN, NAN, NAN, SNAN, SNAN];

impl From<&str> for BID128 {
  /// Creates [BID128] from string.
  fn from(s: &str) -> Self {
    let text = s.trim().to_uppercase();
    match text.as_str() {
      "INFINITY" | "+INFINITY" | "INF" | "+INF" => return P_INF,
      "-INFINITY" | "-INF" => return N_INF,
      "" | "NAN" | "+NAN" => return NAN,
      "SNAN" | "+SNAN" => return SNAN,
      other => {
        for (i, prefix) in PREFIXES.iter().enumerate() {
          if other.starts_with(prefix) {
            return VALUES[i];
          }
        }
      }
    }
    let mut c128 = 0u128;
    let mut exp = 0i32;
    let mut sign = 0u64;
    let mut first_non_zero = false;
    let mut first_decimal_point = false;

    let mut state = 0_usize;
    for ch in text.chars() {
      match state {
        0 => {
          // beginning of the input
          match ch {
            '-' => {
              sign = SIGN_MASK64;
              state = 1;
            }
            '+' | '0' => state = 1,
            '1'..='9' => {
              c128 = c128 * 10 + ((ch as u8) - 48) as u128;
              state = 2;
            }
            _ => {
              //ERROR
            }
          }
        }
        1 => {
          // consuming leading zeros
          match ch {
            '0' => state = 1,
            '1'..='9' => {
              c128 = c128 * 10 + ((ch as u8) - 48) as u128;
              state = 2;
            }
            '.' => {
              first_decimal_point = true;
              state = 2;
            }
            _ => {
              // ERROR
            }
          }
        }
        2 => {
          // consuming digits before `E` character
          match ch {
            '0'..='9' => {
              c128 = c128 * 10 + ((ch as u8) - 48) as u128;
              state = 2;
            }
            _ => {}
          }
        }
        _ => {}
      }
    }

    println!("{}", c128);

    let mut res = __bid128!(0, 0);
    res
  }
}
