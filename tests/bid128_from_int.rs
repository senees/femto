use femto::BID128;

#[inline(always)]
fn eq(expected: &str, actual: BID128) {
  assert_eq!(expected, format!("{:?}", actual))
}

#[test]
fn test_bid128_from_u32() {
  eq("[0,3040000000000000]", BID128::from(0_u32));
  eq("[FFFFFFFF,3040000000000000]", BID128::from(u32::MAX));
}

#[test]
fn test_bid128_from_i32() {
  eq("[80000000,B040000000000000]", BID128::from(i32::MIN));
  eq("[0,3040000000000000]", BID128::from(0_i32));
  eq("[7FFFFFFF,3040000000000000]", BID128::from(i32::MAX));
}

#[test]
fn test_bid128_from_u64() {
  eq("[0,3040000000000000]", BID128::from(0_u64));
  eq("[FFFFFFFFFFFFFFFF,3040000000000000]", BID128::from(u64::MAX));
}

#[test]
fn test_bid128_from_i64() {
  eq("[8000000000000000,B040000000000000]", BID128::from(i64::MIN));
  eq("[0,3040000000000000]", BID128::from(0_i64));
  eq("[7FFFFFFFFFFFFFFF,3040000000000000]", BID128::from(i64::MAX));
}
