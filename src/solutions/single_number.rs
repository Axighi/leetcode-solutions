pub struct Solution {}

impl Solution {
  // a⊕b⊕a=(a⊕a)⊕b=0⊕b=b
  #[allow(dead_code)]
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut a: i32 = 0;

    for x in nums {
      a = a ^ x;
    }

    a
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
  assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
}
