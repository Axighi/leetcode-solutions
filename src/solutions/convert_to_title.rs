pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn convert_to_title(n: i32) -> String {
    let mut ret = String::from("");
    let mut val = n;
    while val > 0 {
      val -= 1;

      ret.insert(0, ('A' as u8 + (val % 26) as u8) as char);
      val /= 26;
    }

    ret
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::convert_to_title(1), "A");
  assert_eq!(Solution::convert_to_title(701), "ZY");
  assert_eq!(Solution::convert_to_title(26), "Z");
  assert_eq!(Solution::convert_to_title(27), "AA");
  assert_eq!(Solution::convert_to_title(52), "AZ");
  assert_eq!(Solution::convert_to_title(702), "ZZ");
}
