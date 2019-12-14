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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = vec![];

    // fn f(node: Option<&Rc<RefCell<TreeNode>>>) {
    let mut f = |node: Option<&Rc<RefCell<TreeNode>>>| {
      let mut level: Vec<i32> = vec![];
      match node {
        Some(_node) => {
          level.push(_node.borrow().val);
        }
        None => (),
      }

      ret.insert(0, level)
    };

    f(root.as_ref());

    return ret;
  }
}
