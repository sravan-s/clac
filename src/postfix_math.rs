use crate::commands::Operation;
use crate::stack::Stack;

fn factorial(n: f64) -> f64 {
  if n < 2.0 {
    1.0
  } else {
    n * factorial(n - 1.0)
  }
}

pub fn compute(mut variables: Stack, mut operators: Stack, mut stash: Stack) -> Stack {
  while variables.len() != 0 && operators.len() != 0 {
    let operator = Operation::from_str(&operators.pop().unwrap());
    match operator {
      Some(Operation::ArithmeticAdd) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1 + op2;
        variables.push(result.to_string());
      },
      Some(Operation::ArithmeticSubstract) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1 - op2;
        variables.push(result.to_string());
      },
      Some(Operation::ArithmeticDivide) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1 / op2;
        variables.push(result.to_string());
      },
      Some(Operation::ArithmeticMultiply) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1 * op2;
        variables.push(result.to_string());
      },
      Some(Operation::Modulo) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1 % op2;
        variables.push(result.to_string());
      },
      Some(Operation::Exponentiation) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.powf(op2);
        variables.push(result.to_string());
      },
      Some(Operation::LogarithmNatural) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.log2();
        variables.push(result.to_string());
      },
      Some(Operation::LogarithmBaseTen) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.log10();
        variables.push(result.to_string());
      },
      Some(Operation::Factorial) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = factorial(op1);
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometrySin) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.sin();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryCos) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.cos();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryTan) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.tan();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryASin) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.asin();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryACos) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.acos();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryATan) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.atan();
        variables.push(result.to_string());
      },
      Some(Operation::TrigonometryATanTwo) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let op2 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.atan2(op2);
        variables.push(result.to_string());
      },
      Some(Operation::ErrorFunction) => {
        // to implement
      },
      Some(Operation::SummationSum) => {
        let mut sum: f64 = 0.0;
        while variables.len() != 0 {
          let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
          sum += op1;
        }
        variables.push(sum.to_string());
      },
      Some(Operation::SummationSubstract) => {
        let mut diff: f64 = 0.0;
        while variables.len() != 0 {
          let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
          diff -= op1;
        }
        variables.push(diff.to_string());
      },
      Some(Operation::RoundingCeil) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.ceil();
        variables.push(result.to_string());
      },
      Some(Operation::RoundingFloor) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.floor();
        variables.push(result.to_string());
      },
      Some(Operation::RoundingRound) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.round();
        variables.push(result.to_string());
      },
      Some(Operation::Absolute) => {
        let op1 = variables.pop().unwrap().parse::<f64>().unwrap();
        let result = op1.abs();
        variables.push(result.to_string());
      },
      Some(Operation::StackSwap) => {
        let op1 = variables.pop().unwrap();
        let op2 = variables.pop().unwrap();
        variables.push(op2);
        variables.push(op1);
      },
      Some(Operation::StackDup) => {
        let op1 = variables.pop().unwrap();
        variables.push(op1.clone());
        variables.push(op1.clone());
      },
      Some(Operation::StackRoll) => {
        // to implement
        // https://www.ugrad.math.ubc.ca/Flat/stack.html
      },
      Some(Operation::StackDrop) => {
        variables.pop();
      },
      Some(Operation::StackClear) => {
        while variables.len() != 0 {
          variables.pop();
        }
      },
      Some(Operation::StackCount) => {
        let len = variables.len();
        variables.push(len.to_string());
      },
      Some(Operation::StashingStashTop) => {
        let op1 = variables.pop().unwrap();
        stash.push(op1);
      },
      Some(Operation::StashingFetchTop) => {
        let op1 = stash.pop().unwrap();
        variables.push(op1);
      },
      Some(Operation::StashingStashAll) => {
        while variables.len() != 0 {
          stash.push(variables.pop().unwrap());
        }
      },
      Some(Operation::StashingFetchAll) => {
        while stash.len() != 0 {
          variables.push(stash.pop().unwrap());
        }
      },
      _ => println!("Done!"),
    }
  }
  println!("variables: {:?}", variables.items);
  println!("operations: {:?}", variables.items);
  println!("stash: {:?}", stash.items);
  Stack::init(10)
}
