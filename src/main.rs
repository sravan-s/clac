use std::env;

mod stack;
use stack::Stack;

mod commands;
use commands::Operation;

mod token_types;
use token_types::TokenType;

fn compute(mut s: Stack) -> Stack {
  let token = s.pop().unwrap();
  println!("{:?}", Operation::is_operation(&token));
  Stack::init(100)
}

fn main() {
  // let mut stash = Stack::init(100);
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
  compute(variable_stack);
}
