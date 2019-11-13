#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
  let mut overflow: i32 = 0;
  let mut i = digits.len();
  let mut digit_sum: i32;
  let mut answer = digits.clone();

  digit_sum = digits[i - 1] + 1i32;

  loop {
    if i == 0 && overflow == 1i32 {
      answer.insert(0, 1i32);
      break;
    }

    if i != digits.len() {
      digit_sum = digits[i - 1] + overflow;
    }

    if digit_sum < 10i32 {
      answer[i - 1] = digit_sum;
      break;
    } else {
      answer[i - 1] = 0i32;
      overflow = 1i32;
      i -= 1;
    }
  }

  return answer;
}

#[test]
fn hooyah() {
  assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
  assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
  assert_eq!(plus_one(vec![9]), vec![1, 0]);
  assert_eq!(plus_one(vec![8, 9, 9, 9]), [9, 0, 0, 0]);
}
