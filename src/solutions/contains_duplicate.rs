pub struct Solution {}

use std::collections::HashMap;

impl Solution {
  #[allow(dead_code)]
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for n in nums {
      match hash.get(&n) {
        Some(_) => return true,
        _ => hash.insert(n, 1),
      };
    }

    false
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
  assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
  assert_eq!(
    Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
    true
  );
}
