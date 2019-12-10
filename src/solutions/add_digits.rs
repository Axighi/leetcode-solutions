pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn add_digits(num: i32) -> i32 {
    let mut _num = num;

    while _num > 9 {
      let str = _num.to_string();
      _num = str.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>() as i32
    }

    _num
  }
}
