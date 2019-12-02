use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  #[allow(dead_code)]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn f(p: Option<&Rc<RefCell<TreeNode>>>, q: Option<&Rc<RefCell<TreeNode>>>) -> bool {
      match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
          let p = p.borrow();
          let q = q.borrow();
          p.val == q.val
            && f(p.left.as_ref(), q.left.as_ref())
            && f(p.right.as_ref(), q.right.as_ref())
        }
        _ => false,
      }
    }
    f(p.as_ref(), q.as_ref())
  }
}
