#[allow(dead_code)]
// Newton's method
pub fn my_sqrt(x: i32) -> i32 {
  if x <= 1 {
    return x;
  }

  let mut ans = x / 2;
  loop {
    let temp = (ans + x / ans) / 2;
    if temp >= ans {
      break;
    }
    ans = temp;
  }

  ans
}

#[test]
fn hooyah() {
  assert_eq!(my_sqrt(4), 2);
  assert_eq!(my_sqrt(8), 2);
}
