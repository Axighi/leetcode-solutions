use std::collections::HashMap;

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let len = nums.len();
    for i in nums {
      let new_count = match map.get(&i) {
        None => 1,
        Some(x) => x + 1,
      };
      if new_count * 2 >= len {
        return i;
      }
      map.insert(i, new_count);
    }
    0
  }
}
