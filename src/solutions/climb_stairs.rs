pub struct Solution {}

impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    if n == 1i32 || n == 2i32 {
      return n;
    }

    let mut dp: Vec<i32> = vec![0; n as usize + 1];
    dp[1] = 1i32;
    dp[2] = 2i32;

    let mut i: usize = 3;
    while i <= n as usize {
      dp[i] = dp[i - 1] + dp[i - 2];
      i += 1;
    }

    return dp[n as usize];
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::climb_stairs(1), 1);
  assert_eq!(Solution::climb_stairs(2), 2);
  assert_eq!(Solution::climb_stairs(3), 3);
  assert_eq!(Solution::climb_stairs(4), 5);
}
