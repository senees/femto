use femto::BID128;

#[inline(always)]
fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, actual.to_string());
}

#[test]
fn test_bid128_to_string_0001() {
  eq("+0E+0", BID128::from(0_u32));
}

#[test]
fn test_bid128_to_string_0002() {
  eq("-4E+0", BID128::from(-4_i32));
}
