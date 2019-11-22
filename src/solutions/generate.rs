pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 0 {
      vec![]
    } else if num_rows == 1 {
      vec![vec![1]]
    } else if num_rows == 2 {
      vec![vec![1], vec![1, 1]]
    } else {
      let mut r: usize = 3;
      let mut triangle = vec![vec![1], vec![1, 1]];
      let mut last_row = vec![1, 1];

      while r <= num_rows as usize {
        let new_row = &mut vec![0; r];
        for (i, x) in new_row.iter_mut().enumerate() {
          if i == 0 || i == r - 1 {
            *x = 1;
          } else {
            *x = last_row[i - 1] + last_row[i];
          }
        }
        triangle.push(new_row.clone());
        last_row = new_row.clone();
        r += 1;
      }

      triangle
    }
  }
}
