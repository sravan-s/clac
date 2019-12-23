use std::env;

mod stack;
use stack::Stack;

mod commands;
use commands::Operation;

fn compute(mut s: Stack) -> Stack {
  let token = s.pop().unwrap();
  println!("{:?}", Operation::is_operation(&token));
  Stack::init(100)
}

fn main() {
  let mut s = Stack::init(100);
  let args: Vec<String> = env::args().collect();
  for arg in args {
    s.push(arg);
  }
  compute(s);
}
