// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[allow(dead_code)]
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn f(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
      if node.is_none() {
        return 0;
      }

      match (
        node.unwrap().borrow().left.as_ref(),
        node.unwrap().borrow().right.as_ref(),
      ) {
        (None, None) => 1,
        (None, Some(right)) => f(Some(right)) + 1,
        (Some(left), None) => f(Some(left)) + 1,
        (Some(left), Some(right)) => cmp::max(f(Some(left)), f(Some(right))) + 1,
      }
    }

    f(root.as_ref())
  }
}
