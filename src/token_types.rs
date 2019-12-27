use crate::commands::Operation;

#[derive(Debug, PartialEq)]
pub enum TokenType {
  Variable,
  Operator,
  Unknown
}

impl TokenType {
  pub fn from_str(token: &str) -> TokenType {
    if token.parse::<f64>().is_ok() {
      TokenType::Variable
    } else if Operation::is_operation(token) {
      TokenType::Operator
    } else {
      TokenType::Unknown
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_unknown_token() {
    let operation = super::TokenType::from_str("token: &str");
    assert_eq!(super::TokenType::Unknown, operation);
  }

  #[test]
  fn test_variable_token() {
    let variable = super::TokenType::from_str("123.44");
    assert_eq!(super::TokenType::Variable, variable);
  }

  #[test]
  fn test_operator_token() {
    let operator = super::TokenType::from_str("+");
    assert_eq!(super::TokenType::Operator, operator);
  }
}
