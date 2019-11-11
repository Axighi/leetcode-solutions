use std::cmp::max;
use std::i32::MIN;

// Kadane's algorithm
#[allow(dead_code)]
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
  let mut best_sum = MIN;
  let mut index = 0;

  // If all minus number, return the largest one.
  for (i, x) in nums.iter().enumerate() {
    best_sum = max(best_sum, *x);
    index = i;
    if *x >= 0 {
      break;
    }
  }

  if index == nums.len() - 1 {
    return best_sum;
  }

  let mut current_sum = 0;

  for x in nums.iter() {
    current_sum = max(0, current_sum + x);
    best_sum = max(best_sum, current_sum);
  }

  return best_sum;
}

#[test]
fn hooyah() {
  assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
  assert_eq!(max_sub_array(vec![-2]), -2);
}
