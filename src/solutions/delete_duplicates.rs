// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[allow(dead_code)]
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

pub struct Solution {}

impl Solution {
  #[allow(dead_code)]
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head == None {
      return None;
    }

    let mut _head = head;
    let mut b1 = _head.as_mut().unwrap();

    while let Some(b2) = b1.next.as_mut() {
      if b2.val != b1.val {
        b1 = b1.next.as_mut().unwrap();
        continue;
      }
      let b2_next = std::mem::replace(&mut b2.next, None);
      std::mem::replace(&mut b1.next, b2_next);
    }

    _head
  }
}
