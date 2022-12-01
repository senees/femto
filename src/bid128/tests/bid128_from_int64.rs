//! bid128_from_int64

use crate::bid128::bid_functions::ExceptionStatusFlag;
use crate::BID128;

#[inline(always)]
fn eq(n: i64, result: &str, status: ExceptionStatusFlag) {
  let actual = BID128::from(n);
  assert_eq!(result, format!("[{:016x}{:016x}]", actual.w[1], actual.w[0]));
  assert_eq!(status, actual.flags);
}

#[test]
fn _0001() {
  eq(-1297714641311568497_i64, "[b0400000000000001202689737331271]", 0x00);
}

#[test]
fn _0002() {
  eq(-1382533918512158604, "[b040000000000000132fbf461ed8978c]", 0x00);
}

#[test]
fn _0003() {
  eq(1991200058281483048, "[30400000000000001ba229e7369adb28]", 0x00);
}

#[test]
fn _0004() {
  eq(-1998782831704824207, "[b0400000000000001bbd1a653aea598f]", 0x00);
}

#[test]
fn _0005() {
  eq(2081344241858484150, "[30400000000000001ce26b8f7f4797b6]", 0x00);
}

#[test]
fn _0006() {
  eq(-3494168784785967941, "[b040000000000000307dc7ff326d0b45]", 0x00);
}

#[test]
fn _0007() {
  eq(-3623475514305027548, "[b04000000000000032492bc8427a1ddc]", 0x00);
}

#[test]
fn _0008() {
  eq(-3719337100671242603, "[b040000000000000339dbd631d70d16b]", 0x00);
}

#[test]
fn _0009() {
  eq(4134085355817711631, "[3040000000000000395f38ba50a8b80f]", 0x00);
}

#[test]
fn _0010() {
  eq(-4543384426972292615, "[b0400000000000003f0d58107fb9a607]", 0x00);
}

#[test]
fn _0011() {
  eq(-456313642285820771, "[b040000000000000065526d70a9c6b63]", 0x00);
}

#[test]
fn _0012() {
  eq(572333114425411975, "[304000000000000007f155ef6a850987]", 0x00);
}

#[test]
fn _0013() {
  eq(-5896812445967530814, "[b04000000000000051d5afad4cff3b3e]", 0x00);
}

#[test]
fn _0014() {
  eq(6084829525942336602, "[30400000000000005471a8370c99fc5a]", 0x00);
}

#[test]
fn _0015() {
  eq(6343298205297819548, "[30400000000000005807ec1c07263f9c]", 0x00);
}

#[test]
fn _0016() {
  eq(-640590259490231037, "[b04000000000000008e3d5726f7d92fd]", 0x00);
}

#[test]
fn _0017() {
  eq(7071067091834809443, "[304000000000000062217a0e4fc4ac63]", 0x00);
}

#[test]
fn _0018() {
  eq(7575924962073745274, "[30400000000000006923179d316b977a]", 0x00);
}

#[test]
fn _0019() {
  eq(-8534495841839544214, "[b04000000000000076709ec01de4f796]", 0x00);
}

#[test]
fn _0020() {
  eq(945634496444636363, "[30400000000000000d1f919077f01ccb]", 0x00);
}
