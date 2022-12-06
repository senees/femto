use femto::BID128;

#[inline(always)]
fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, format!("{:?}", actual));
}

#[test]
fn test_bid128_from_u32() {
  eq("[3040000000000000,0000000000000000]", BID128::from(0_u32));
  eq("[3040000000000000,00000000ffffffff]", BID128::from(u32::MAX));
}

#[test]
fn test_bid128_from_i32() {
  eq("[b040000000000000,0000000080000000]", BID128::from(i32::MIN));
  eq("[3040000000000000,0000000000000000]", BID128::from(0_i32));
  eq("[3040000000000000,000000007fffffff]", BID128::from(i32::MAX));
}

#[test]
fn test_bid128_from_u64() {
  eq("[3040000000000000,0000000000000000]", BID128::from(0_u64));
  eq("[3040000000000000,ffffffffffffffff]", BID128::from(u64::MAX));
}

#[test]
fn test_bid128_from_i64() {
  eq("[b040000000000000,8000000000000000]", BID128::from(i64::MIN));
  eq("[3040000000000000,0000000000000000]", BID128::from(0_i64));
  eq("[3040000000000000,7fffffffffffffff]", BID128::from(i64::MAX));
}
