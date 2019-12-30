use std::env;

mod stack;
use stack::Stack;

mod commands;

mod token_types;
use token_types::TokenType;

mod postfix_math;
use postfix_math::compute;

fn main() {
  let stash = Stack::init(100);
  let mut variable_stack = Stack::init(100);
  let mut operator_stack = Stack::init(100);
  let mut args: Vec<String> = env::args().collect();
  // removes first arg - which is usually path of app binary
  args.remove(0);
  println!("args: {:?}", args);
  for arg in args {
    let token_type = TokenType::from_str(&arg);
    match token_type {
      TokenType::Operator => operator_stack.push(arg),
      TokenType::Variable => variable_stack.push(arg),
      _ => panic!("Unknown type {:?}", arg),
    };
  }
  compute(variable_stack, operator_stack, stash);
}
