pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut _nums = nums.clone();
    let mut duplicates = vec![];

    for i in 0.._nums.len() {
      let m = _nums[i as usize].abs() - 1;
      _nums[m as usize] = -_nums[m as usize];
      if _nums[m as usize] > 0 {
        duplicates.push(m as i32 + 1)
      }
    }

    return duplicates;
  }
}

#[test]
fn hooyah() {
  assert_eq!(
    Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
    vec![2, 3]
  )
}
