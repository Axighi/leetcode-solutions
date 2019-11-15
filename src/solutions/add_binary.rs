fn and(x: char, y: char, overflow: bool) -> (char, bool) {
  if overflow {
    if x == '1' && y == '1' {
      ('1', true)
    } else if (x == '1' && y == '0') || (x == '0' && y == '1') {
      ('0', true)
    } else {
      ('1', false)
    }
  } else {
    if x == '1' && y == '1' {
      ('0', true)
    } else if (x == '1' && y == '0') || (x == '0' && y == '1') {
      ('1', false)
    } else {
      ('0', false)
    }
  }
}

fn get_char(v: &Vec<char>, index: usize) -> char {
  *v.get(index).unwrap_or(&'0')
}

#[allow(dead_code)]
pub fn add_binary(a: String, b: String) -> String {
  let mut overflow = false;
  let mut answer = String::from("");
  let long: String;
  let short: String;

  if a.len() >= b.len() {
    long = a;
    short = b;
  } else {
    long = b;
    short = a;
  }

  let mut long_vec: Vec<char> = long.chars().collect();
  let mut short_vec: Vec<char> = short.chars().collect();

  long_vec.reverse();
  short_vec.reverse();

  let mut i = 0;

  while i < long.len() {
    let (res, _overflow) = and(get_char(&long_vec, i), get_char(&short_vec, i), overflow);
    answer.insert(0, res);
    overflow = _overflow;
    i += 1;
  }

  if overflow {
    answer.insert(0, '1');
  }

  answer
}

#[test]
fn hooyah() {
  assert_eq!(add_binary("11".to_string(), "1".to_string()), "100");
  assert_eq!(add_binary("1010".to_string(), "1011".to_string()), "10101");
}
