pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn get_row(row_index: i32) -> Vec<i32> {
    if row_index == 0 {
      vec![1]
    } else if row_index == 1 {
      vec![1, 1]
    } else {
      let mut r: usize = 3;
      let mut last_row = vec![1, 1];

      while r <= (row_index + 1) as usize {
        let new_row = &mut vec![0; r];
        for (i, x) in new_row.iter_mut().enumerate() {
          if i == 0 || i == r - 1 {
            *x = 1;
          } else {
            *x = last_row[i - 1] + last_row[i];
          }
        }
        last_row = new_row.clone();
        r += 1;
      }

      last_row
    }
  }
}

#[test]
fn hooyah() {
  assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
}
