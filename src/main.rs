mod solutions;

// use crate::solutions::sqrt_x::*;
// use crate::solutions::climb_stairs::*;
use crate::solutions::merge::*;

fn main() {
  let mut nums1 = vec![1, 2, 3, 0, 0, 0];
  Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
  println!("{:?}", nums1);
}
