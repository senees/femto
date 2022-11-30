//!

macro_rules! l0_normalize_10to18 {
  ($x_hi:expr, $x_lo:expr) => {{
    let l0_tmp = $x_lo + BID_TWOTO60_M_10TO18;
    if l0_tmp & BID_TWOTO60 > 0 {
      $x_hi += $x_hi;
      $x_lo = (l0_tmp << 4) >> 4;
    }
  }};
}

macro_rules! l0_split_midi_2 {
  ($x:expr, $ptr:expr, $midi:expr) => {{
    let mut l0_head = $x >> 10;
    let mut l0_tail = ($x & 0x03FF) + (l0_head << 5) - (l0_head << 3);
    let l0_tmp = l0_tail >> 10;
    l0_head += l0_tmp;
    l0_tail = (l0_tail & 0x03FF) + (l0_tmp << 5) - (l0_tmp << 3);
    if l0_tail > 999 {
      l0_tail -= 1000;
      l0_head += 1;
    }
    $midi[$ptr] = l0_head;
    $ptr += 1;
    $midi[$ptr] = l0_tail;
    $ptr += 1;
  }};
}

macro_rules! l0_split_midi_3 {
  ($x:expr, $ptr:expr, $midi:expr) => {{
    let mut l0_x = $x as u32;
    let mut l0_head = ((l0_x >> 17) * 34359) >> 18;
    l0_x -= l0_head * 1000000;
    if l0_x >= 1000000 {
      l0_x -= 1000000;
      l0_head += 1;
    }
    let mut l0_mid = l0_x >> 10;
    let mut l0_tail = (l0_x & 0x03FF) + (l0_mid << 5) - (l0_mid << 3);
    let l0_tmp = (l0_tail) >> 10;
    l0_mid += l0_tmp;
    l0_tail = (l0_tail & 0x3FF) + (l0_tmp << 5) - (l0_tmp << 3);
    if (l0_tail > 999) {
      l0_tail -= 1000;
      l0_mid += 1;
    }
    $midi[$ptr] = l0_head;
    $ptr += 1;
    $midi[$ptr] = l0_mid;
    $ptr += 1;
    $midi[$ptr] = l0_tail;
    $ptr += 1;
  }};
}

macro_rules! l1_split_midi_6 {
  ($x:expr, $ptr:expr, $midi:expr) => {{
    let mut l1_xhi_64 = (($x >> 28) * BID_INV_TENTO9) >> 33;
    let mut l1_xlo_64 = $x - l1_xhi_64 * (BID_TENTO9 as u64);
    if l1_xlo_64 >= (BID_TENTO9 as u64) {
      l1_xlo_64 -= (BID_TENTO9 as u64);
      l1_xhi_64 += 1;
    }
    let l1_x_hi = l1_xhi_64 as u32;
    let l1_x_lo = l1_xlo_64 as u32;
    l0_split_midi_3!(l1_x_hi, $ptr, $midi);
    l0_split_midi_3!(l1_x_lo, $ptr, $midi);
  }};
}

macro_rules! l1_split_midi_6_lead {
  ($x:expr, $ptr:expr, $midi:expr) => {{
    if $x >= (BID_TENTO9 as u64) {
      let mut l1_xhi_64 = (($x >> 28) * BID_INV_TENTO9) >> 33;
      let mut l1_xlo_64 = $x - l1_xhi_64 * (BID_TENTO9 as u64);
      if l1_xlo_64 >= (BID_TENTO9 as u64) {
        l1_xlo_64 -= (BID_TENTO9 as u64);
        l1_xhi_64 += 1;
      }
      let l1_x_hi = l1_xhi_64 as u32;
      let l1_x_lo = l1_xlo_64 as u32;
      if l1_x_hi >= BID_TENTO6 {
        l0_split_midi_3!(l1_x_hi, $ptr, $midi);
        l0_split_midi_3!(l1_x_lo, $ptr, $midi);
      } else if l1_x_hi >= BID_TENTO3 {
        l0_split_midi_2!(l1_x_hi, $ptr, $midi);
        l0_split_midi_3!(l1_x_lo, $ptr, $midi);
      } else {
        $midi[$ptr] = l1_x_hi;
        $ptr += 1;
        l0_split_midi_3!(l1_x_lo, $ptr, $midi);
      }
    } else {
      let l1_x_lo = $x as u32;
      if l1_x_lo >= BID_TENTO6 {
        l0_split_midi_3!(l1_x_lo, $ptr, $midi);
      } else if l1_x_lo >= BID_TENTO3 {
        l0_split_midi_2!(l1_x_lo, $ptr, $midi);
      } else {
        $midi[$ptr] = l1_x_lo;
        $ptr += 1;
      }
    }
  }};
}

macro_rules! l0_midi2str {
  ($x:expr, $str:expr) => {{
    let l0_src = BID_MIDI_TBL[$x as usize];
    let _ = write!(&mut $str, "{}", l0_src);
  }};
}

macro_rules! l0_midi2str_lead {
  ($x:expr, $str:expr) => {{
    let l0_src = BID_MIDI_TBL[$x as usize];
    if $x >= 100 {
      let _ = write!(&mut $str, "{}", l0_src);
    } else if $x >= 10 {
      let _ = write!(&mut $str, "{}", &l0_src[1..]);
    } else {
      let _ = write!(&mut $str, "{}", &l0_src[2..]);
    }
  }};
}

/// Convenient macro for converting ASCII codes to [char] type.
macro_rules! ascii_to_char {
  ($x:expr) => {{
    ($x as u8) as char
  }};
}

pub(crate) use {ascii_to_char, l0_midi2str, l0_midi2str_lead, l0_normalize_10to18, l0_split_midi_2, l0_split_midi_3, l1_split_midi_6, l1_split_midi_6_lead};
