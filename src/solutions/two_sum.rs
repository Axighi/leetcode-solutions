pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut low: usize = 0;
    let mut high: usize = numbers.len() - 1;

    while low < high {
      let sum: i32 = numbers[low] + numbers[high];
      if sum == target {
        return vec![(low + 1) as i32, (high + 1) as i32];
      } else if sum < target {
        low += 1;
      } else {
        high -= 1;
      }
    }
    vec![-1, -1]
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
