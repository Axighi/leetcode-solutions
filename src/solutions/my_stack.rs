struct MyStack {
   stack: Vec[i32],
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
      stack = vec![];
      &self
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
      self.stack.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&self) -> i32 {
      self.stack.pop();
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
      self.stack.top();
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
      self.stack.len() == 0
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

 #[test]
 fn hooyah() {
  let obj = MyStack::new();
  obj.push(x);
  let ret_2: i32 = obj.pop();
  let ret_3: i32 = obj.top();
  let ret_4: bool = obj.empty();
 }