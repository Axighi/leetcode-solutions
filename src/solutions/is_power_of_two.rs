pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
      return false;
    }

    let mut k = n;

    while k != 0 {
      if k % 2 != 0 && k != 1 {
        return false;
      }
      k = k >> 1;
    }

    true
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::is_power_of_two(0), false);
  assert_eq!(Solution::is_power_of_two(1), true);
  assert_eq!(Solution::is_power_of_two(3), false);
  assert_eq!(Solution::is_power_of_two(16), true);
  assert_eq!(Solution::is_power_of_two(218), false);
}
