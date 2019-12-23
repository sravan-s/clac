/// ## Mathematical operations on stack
/// #### Arithmetic operations
/// + Pop two values a and b and push the result of a + b.
/// - Pop two values a and b and push the result of b - a.
/// * Pop two values a and b and push the result of a * b.
/// / Pop two values a and b and push the result of b / a.

/// #### Modulo operation
/// % Pop two values a and b and push the remainder of the Euclidean division of b by a.

/// #### Exponentiation
/// ^ Pop two values a and b and push the result of b ^ a.

/// #### Logarithm
/// ln Pop the value a and push its natural logarithm.
/// log Pop the value a and push its logarithm to base 10.

/// #### Factorial
/// ! Pop the value a and push its factorial.

/// #### Trigonometry
/// sin Pop the value a and push its sine.
/// cos Pop the value a and push its cosine.
/// tan Pop the value a and push its tangent.
/// asin Pop the value a and push its arc sine.
/// acos Pop the value a and push its arc cosine.
/// atan Pop the value a and push its arc tangent.
/// atan2 Pop two values a and b and push the arc tangent of b / a, using the signs of a and b to determine the quadrant.

/// #### Error function
/// erf Pop the value a and push its error function.

/// #### Summation
/// sum Pop all the values in the stack and push their sum.
/// add Pop the value a and remove that many items from the stack. Push their sum.

/// #### Rounding
/// ceil Pop the value a and push the smallest integer value greater than or equal to a.
/// floor Pop the value a and push the largest integer value less than or equal to a.
/// round Pop the value a and push the integer value closest to a.

/// #### Absolute value
/// abs Pop the value a and push the non-negative value of a.

/// #### Stack manipulation
/// swap Pop two values a and b and push the values a, b.
/// dup Pop the value a and push the values a, a.
/// roll Pop two values a and b and rotate b elements in the stack a times.
/// drop Remove the top of the stack.
/// clear Remove all the elements in the stack.
/// count Push the number of items in the stack.
/// _ Push on the stack the result of the last operation.

/// #### Stashing
/// stash Pop the value a and move that many items to the stash.
/// fetch Pop the value a and move that many items from the stash.
/// . Stash the top of the stack.
/// , Fetch one stashed item.
/// : Stash all the items in the stack.
/// ; Fetch all stashed items
#[derive(Debug, PartialEq)]
pub enum Operation {
  ArithmeticAdd,
  ArithmeticSubstract,
  ArithmeticMultiply,
  ArithmeticDivide,
  Modulo,
  Exponentiation,
  LogarithmNatural,
  LogarithmBaseTen,
  Factorial,
  TrigonometrySin,
  TrigonometryCos,
  TrigonometryTan,
  TrigonometryASin,
  TrigonometryACos,
  TrigonometryATan,
  TrigonometryATanTwo,
  ErrorFunction,
  SummationSum,
  SummationSubstract,
  RoundingCeil,
  RoundingFloor,
  RoundingRound,
  Absolute,
  StackSwap,
  StackDup,
  StackRoll,
  StackDrop,
  StackClear,
  StackCount,
  StackPush,
  StashingStashTop,
  StashingFetchTop,
  StashingStashAll,
  StashingFetchAll,
}

impl Operation {
  // Todo - implement as trait
  // https://github.com/rust-lang/rust/issues/56517
  /// Convert a string into an Operation
  pub fn from_str(token: &str) -> Option<Operation> {
    match token {
      "+" => Some(Operation::ArithmeticAdd),
      "-" => Some(Operation::ArithmeticSubstract),
      "*" => Some(Operation::ArithmeticMultiply),
      "/" => Some(Operation::ArithmeticDivide),
      "%" => Some(Operation::Modulo),
      "^" => Some(Operation::Exponentiation),
      "ln" => Some(Operation::LogarithmNatural),
      "log" => Some(Operation::LogarithmBaseTen),
      "!" => Some(Operation::Factorial),
      "sin" => Some(Operation::TrigonometrySin),
      "cos" => Some(Operation::TrigonometryCos),
      "tan" => Some(Operation::TrigonometryTan),
      "asin" => Some(Operation::TrigonometryASin),
      "acos" => Some(Operation::TrigonometryACos),
      "atan" => Some(Operation::TrigonometryATan),
      "atan2" => Some(Operation::TrigonometryATanTwo),
      "erf" => Some(Operation::ErrorFunction),
      "sum" => Some(Operation::SummationSum),
      "add" => Some(Operation::SummationSubstract),
      "ceil" => Some(Operation::RoundingCeil),
      "floor" => Some(Operation::RoundingFloor),
      "round" => Some(Operation::RoundingRound),
      "abs" => Some(Operation::Absolute),
      "swap" => Some(Operation::StackSwap),
      "dup" => Some(Operation::StackDup),
      "roll" => Some(Operation::StackRoll),
      "drop" => Some(Operation::StackDrop),
      "clear" => Some(Operation::StackClear),
      "count" => Some(Operation::StackCount),
      "_" => Some(Operation::StackPush),
      "." => Some(Operation::StashingStashTop),
      "," => Some(Operation::StashingFetchTop),
      ":" => Some(Operation::StashingStashAll),
      ";" => Some(Operation::StashingFetchAll),
      _ => None,
    }
  }

  /// Checks if an string is an operation
  pub fn is_operation(token: &str) -> bool {
    let op = Operation::from_str(&token);
    match op {
      Some(_o) => true,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_unknown_operation() {
    let operation = super::Operation::is_operation("token: &str");
    assert_eq!(false, operation);
  }

  #[test]
  fn test_known_operation() {
    let operation = super::Operation::is_operation("+");
    assert_eq!(true, operation);
  }
}
