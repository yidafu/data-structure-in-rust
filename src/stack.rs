use std::vec;


struct Stack<T> {
  top: usize,
  data: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Stack<T> {
    Stack {
      top: 0,
      data: vec![],
    }
  }

  fn push(&mut self, val: T) {
    self.data.push(val);
    self.top += 1;
  }

  fn pop(&mut self) -> Option<T> {
    if self.is_empty() { return None; }
    self.top -= 1;
    self.data.pop()
  }

  fn peek(&self) -> Option<&T> {
    self.data.get(self.top)
  }

  fn is_empty(&self) -> bool {
    self.top == 0
  }

  fn size(&self) -> usize {
    self.top
  }
}

#[test]

fn test_bracket_match() {

  fn bracket_match(brackets: &str) -> bool {

    let mut stack = Stack::new();

    let char_list: Vec<char> = brackets.chars().collect();
    for c in char_list {
      if c == '(' {
        stack.push(c);
      } else {
        if stack.is_empty() {
          return false;
        } else {
          stack.pop();
        }
      }
    }
    stack.is_empty()
  }

  assert!(bracket_match("(())"), "(()) should return true");

  assert!(!bracket_match("(()"), "(() should return false");
  assert!(!bracket_match("())"), "()) should return false");
}