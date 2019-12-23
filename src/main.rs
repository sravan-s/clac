use std::env;

mod stack;
use stack::Stack;

fn main() {
  let mut s = Stack::init(10);
  s.push(String::from("a"));
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  println!("{:?}", s.pop());
}
