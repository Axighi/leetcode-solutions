pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    assert_eq!(n, n);
    let mut insert_pos: usize = 0;
    let mut _m = m as usize;

    for num2 in nums2.iter() {
      if m == 0 {
        nums1.insert(insert_pos, *num2);
        nums1.pop();
        insert_pos += 1;
      } else {
        loop {
          if _m == insert_pos {
            nums1.insert(insert_pos, *num2);
            _m += 1;
            nums1.pop();
            break;
          }

          if nums1[insert_pos] >= *num2 {
            nums1.insert(insert_pos, *num2);
            _m += 1;
            nums1.pop();
            break;
          } else {
            insert_pos += 1;
          }
        }
      }
    }
  }
}

#[test]
fn hooyah() {
  let mut nums1 = vec![1, 2, 3, 0, 0, 0];
  Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
  assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

  let mut nums2 = vec![0];
  Solution::merge(&mut nums2, 0, &mut vec![1], 1);
  assert_eq!(nums2, vec![1]);

  let mut nums3 = vec![2, 0];
  Solution::merge(&mut nums3, 1, &mut vec![1], 1);
  assert_eq!(nums3, vec![1, 2]);

  let mut nums4 = vec![4, 5, 6, 0, 0, 0];
  Solution::merge(&mut nums4, 3, &mut vec![1, 2, 3], 3);
  assert_eq!(nums4, vec![1, 2, 3, 4, 5, 6]);
}
