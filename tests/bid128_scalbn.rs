use femto::BID128;

#[inline(always)]
fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, actual.to_string());
}

#[test]
fn test_bid128_scalbn_0001() {
  eq("+1034E-2", BID128::from(1034_u64).scalbn(-2));
}
