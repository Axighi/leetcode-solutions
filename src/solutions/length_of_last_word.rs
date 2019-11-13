#[allow(dead_code)]
pub fn length_of_last_word(s: String) -> i32 {
  if s.len() == 0 {
    return 0;
  }

  let mut i: i32 = (s.len() - 1) as i32;
  let mut ans = 0;
  let _s = s.as_bytes();

  while i >= 0 {
    // 32 is ' ' as byte
    if ans == 0 && _s[i as usize] == 32 {
      i -= 1;
    } else if _s[i as usize] != 32 {
      ans += 1;
      i -= 1;
    } else {
      break;
    }
  }

  return ans;
}

#[test]
fn hooyah() {
  assert_eq!(length_of_last_word("hello world".to_string()), 5);
  assert_eq!(length_of_last_word(" ".to_string()), 0);
  assert_eq!(length_of_last_word("a ".to_string()), 1);
}
