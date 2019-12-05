pub mod operations;
pub use operations::*;

pub mod utils;
pub use utils::*;

use std::panic;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
  match panic::catch_unwind(|| solve_with_params(input, 1)) {
    Ok(code) => code as i32,
    Err(_) => -1,
  }
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
  match panic::catch_unwind(|| solve_with_params(input, 5)) {
    Ok(code) => code as i32,
    Err(_) => -1,
  }
}

pub fn solve_with_params(input: &Vec<i32>, initial: i32) -> usize {
  let mut opcodes = input.clone();
  let mut index = 0;
  loop {
    if let [opcode, operand_one, operand_two, dest] = opcodes[index..index + 4] {
      let operation = Operation::parse(opcode, operand_one, operand_two, dest as usize, &opcodes);
      let result = operation.operate();

      match operation {
        Operation::Sum(_, _, op_dest) => opcodes[op_dest] = result,
        Operation::Multiply(_, _, op_dest) => opcodes[op_dest] = result,
        Operation::Store(value) => opcodes[value] = initial,
        Operation::Print(value) => println!("{}", value),
        Operation::JumpIfTrue(_, dest) => {
          if result as usize == dest {
            index = dest;
          } else {
            index += 3;
          }
        }
        Operation::JumpIfFalse(_, dest) => {
          if result as usize == dest {
            index = dest;
          } else {
            index += 3;
          }
        }
        Operation::LessThan(_, _, dest) => {
          if result as usize == dest {
            opcodes[dest] = 1
          } else {
            opcodes[dest] = 0
          }
        }
        Operation::Equals(_, _, dest) => {
          if result as usize == dest {
            opcodes[dest] = 1
          } else {
            opcodes[dest] = 0
          }
        }
      };
      index += operation.length();
    } else {
      return opcodes[0] as usize;
    }
  }
}
