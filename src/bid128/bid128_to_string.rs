use crate::BID128;

use crate::bid128::bid128_to_string_macros::*;
use crate::bid128::bid128_to_string_tables::*;
use std::fmt::Write;

impl ToString for BID128 {
  /// Converts [BID128] into [string].
  fn to_string(&self) -> String {
    if (self.w[1] & 0x7800000000000000_u64) == 0x7800000000000000_u64 {
      if (self.w[1] & 0x7C00000000000000_u64) == 0x7C00000000000000_u64 {
        if (self.w[1] & 0x7E00000000000000_u64) == 0x7E00000000000000_u64 {
          format!("{}SNaN", if (self.w[1] as i64) < 0 { '-' } else { '+' })
        } else {
          format!("{}NaN", if (self.w[1] as i64) < 0 { '-' } else { '+' })
        }
      } else if (self.w[1] & 0x8000000000000000_u64) == 0x0_u64 {
        "+Inf".to_string()
      } else {
        "-Inf".to_string()
      }
    } else if ((self.w[1] & 0x0001FFFFFFFFFFFF_u64) == 0x0_u64) && (self.w[0] == 0x0_u64) {
      let mut str = String::with_capacity(100);
      if self.w[1] & 0x8000000000000000_u64 != 0_u64 {
        let _ = write!(&mut str, "-");
      } else {
        let _ = write!(&mut str, "+");
      }
      let _ = write!(&mut str, "0E");
      let mut exp = (((self.w[1] & 0x7FFE000000000000_u64) >> 49) - 6176) as i32;
      if exp > (((0x5ffe) >> 1) - (6176)) {
        exp = ((((self.w[1] << 2) & 0x7FFE000000000000_u64) >> 49) - 6176) as i32;
      }
      if exp >= 0 {
        let _ = write!(&mut str, "+{}", exp);
      } else {
        let _ = write!(&mut str, "{}", exp);
      }
      str
    } else {
      let mut str = String::with_capacity(100);
      let x_sign = self.w[1] & 0x8000000000000000_u64;
      let mut x_exp = self.w[1] & 0x7FFE000000000000_u64;
      if (self.w[1] & 0x6000000000000000_u64) == 0x6000000000000000_u64 {
        x_exp = (self.w[1] << 2) & 0x7FFE000000000000_u64;
      }
      let c1 = BID128 {
        w: [self.w[0], self.w[1] & 0x0001FFFFFFFFFFFF_u64],
      };
      let mut exp = ((x_exp >> 49) - 6176) as i32;
      if x_sign != 0 {
        let _ = write!(&mut str, "-");
      } else {
        let _ = write!(&mut str, "+");
      }
      if c1.w[1] > 0x0001ED09BEAD87C0_u64
        || (c1.w[1] == 0x0001ED09BEAD87C0_u64 && (c1.w[0] > 0x378D8E63FFFFFFFF_u64))
        || (self.w[1] & 0x6000000000000000_u64) == 0x6000000000000000_u64
        || (c1.w[1] == 0) && (c1.w[0] == 0)
      {
        let _ = write!(&mut str, "0");
      } else {
        let mut tmp = c1.w[0] >> 59;
        let mut lo_18dig = (c1.w[0] << 5) >> 5;
        tmp += c1.w[1] << 5;
        let mut hi_18dig = 0_u64;
        let mut k_lcv = 0;
        while tmp > 0 {
          let mut midi_ind = (tmp & 0x000000000000003F) as usize;
          midi_ind <<= 1;
          tmp >>= 6;
          hi_18dig += BID_MOD10_18_TBL[k_lcv][midi_ind];
          midi_ind += 1;
          lo_18dig += BID_MOD10_18_TBL[k_lcv][midi_ind];
          k_lcv += 1;
          l0_normalize_10to18!(hi_18dig, lo_18dig);
        }
        let mut ptr = 0_usize;
        let mut midi: [u32; 12] = [0; 12];
        if hi_18dig == 0 {
          l1_split_midi_6_lead!(lo_18dig, ptr, midi);
        } else {
          l1_split_midi_6_lead!(hi_18dig, ptr, midi);
          l1_split_midi_6!(lo_18dig, ptr, midi);
        }
        // convert the `midi` into character strings
        l0_midi2str_lead!(midi[0], str);
        k_lcv = 1;
        while k_lcv < ptr {
          l0_midi2str!(midi[k_lcv], str);
          k_lcv += 1;
        }
      }
      // print `E` character and the sign of the exponent
      let _ = write!(&mut str, "E");
      if exp < 0 {
        exp = -exp;
        let _ = write!(&mut str, "-");
      } else {
        let _ = write!(&mut str, "+");
      }
      // determine exponent's representation as a decimal string
      // d0 = exp / 1000;
      let d0 = (exp * 0x418A) >> 24; // 0x418a * 2^-24 = (10^(-3))
      let d123 = exp - 1000 * d0;
      if d0 > 0 {
        // 1000 <= exp <= 6144 => 4 digits to return
        let _ = write!(&mut str, "{}", ascii_to_char!(d0 + 0x30)); // ASCII for decimal digit d0
        let ind = (3 * d123) as usize;
        let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind]);
        let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind + 1]);
        let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind + 2]);
      } else {
        // 0 <= exp <= 999 => d0 = 0
        if d123 < 10 {
          // 0 <= exp <= 9 => 1 digit to return
          let _ = write!(&mut str, "{}", ascii_to_char!(d123 + 0x30)); // ASCII
        } else if d123 < 100 {
          // 10 <= exp <= 99 => 2 digits to return
          let ind = (2 * (d123 - 10)) as usize;
          let _ = write!(&mut str, "{}", BID_CHAR_TABLE2[ind]);
          let _ = write!(&mut str, "{}", BID_CHAR_TABLE2[ind + 1]);
        } else {
          // 100 <= exp <= 999 => 3 digits to return
          let ind = (3 * d123) as usize;
          let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind]);
          let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind + 1]);
          let _ = write!(&mut str, "{}", BID_CHAR_TABLE3[ind + 2]);
        }
      }
      str
    }
  }
}
