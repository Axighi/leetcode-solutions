// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
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
use std::rc::Rc;

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
      return true;
    }

    fn f(t1: Option<&Rc<RefCell<TreeNode>>>, t2: Option<&Rc<RefCell<TreeNode>>>) -> bool {
      match (t1, t2) {
        (None, None) => true,
        (Some(left), Some(right)) => {
          let left = left.borrow();
          let right = right.borrow();
          left.val == right.val
            && f(left.left.as_ref(), right.right.as_ref())
            && f(left.right.as_ref(), right.left.as_ref())
        }
        _ => false,
      }
    }

    f(root.as_ref(), root.as_ref())
  }
}
