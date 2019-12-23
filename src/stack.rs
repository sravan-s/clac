///
/// Creates a stack data structure
///
pub struct Stack {
  /// Max size of the stack
  max_size: usize,
  /// No:of items in stack
  items: Vec<String>,
}

impl Stack {
  /// To initialize a stack
  pub fn init(max_size: usize) -> Self {
    Self {
      max_size,
      items: Vec::with_capacity(max_size),
    }
  }

  /// Push items to stack returns true if successful
  pub fn push(&mut self, item: String) -> bool {
    if self.items.len() == self.max_size {
      return false;
    }
    self.items.push(item);
    true
  }

  /// Pop items from stack
  pub fn pop(&mut self) -> Option<String> {
    self.items.pop()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_init_a_stack() {
    let stack = super::Stack::init(10);
    assert_eq!(10, stack.items.capacity());
  }

  #[test]
  fn test_push_value_success() {
    let size = 2;
    let mut stack = super::Stack::init(size);
    assert_eq!(stack.push(String::from("a")), true);
    stack.push(String::from("a"));
    let mut items: (Vec<String>) = Vec::with_capacity(size);
    items.push(String::from("a"));
    items.push(String::from("a"));
    assert_eq!(stack.items, items);
  }

  #[test]
  fn test_push_value_failure() {
    let size = 1;
    let mut stack = super::Stack::init(size);
    stack.push(String::from("a"));
    assert_eq!(stack.push(String::from("b")), false);
  }

  #[test]
  fn test_pop_value_success() {
    let size = 1;
    let mut stack = super::Stack::init(size);
    stack.push(String::from("a"));
    let poppped = match stack.pop() {
      Some(val) => val,
      None => String::from("")
    };
    assert_eq!(poppped, String::from("a"));
  }

  #[test]
  fn test_pop_value_failure() {
    let size = 1;
    let mut stack = super::Stack::init(size);
    let poppped = match stack.pop() {
      Some(val) => val,
      None => String::from("")
    };
    assert_eq!(poppped, String::from(""));
  }
}
