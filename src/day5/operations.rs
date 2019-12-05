pub use super::utils::digits;

#[derive(Debug)]
pub enum Operation {
  Multiply(i32, i32, usize),
  Sum(i32, i32, usize),
  Store(usize),
  Print(i32),
  JumpIfTrue(i32, usize),
  JumpIfFalse(i32, usize),
  LessThan(i32, i32, usize),
  Equals(i32, i32, usize),
}

impl Operation {
  pub fn operate(&self) -> i32 {
    match self {
      Self::Sum(operand_one, operand_two, _) => operand_one + operand_two,
      Self::Multiply(operand_one, operand_two, _) => operand_one * operand_two,
      Self::Store(operand_one) => *operand_one as i32,
      Self::Print(operand_one) => *operand_one,
      Self::JumpIfTrue(number, index_jump) => {
        if *number != 0 {
          *index_jump as i32
        } else {
          -1
        }
      }
      Self::JumpIfFalse(number, index_jump) => {
        if *number == 0 {
          *index_jump as i32
        } else {
          -1
        }
      }
      Self::LessThan(operand_one, operand_two, dest) => {
        if *operand_one < *operand_two {
          *dest as i32
        } else {
          -1
        }
      }
      Self::Equals(operand_one, operand_two, dest) => {
        if *operand_one == *operand_two {
          *dest as i32
        } else {
          -1
        }
      }
    }
  }

  pub fn length(&self) -> usize {
    match self {
      Self::Sum(_, _, _) => 4,
      Self::Multiply(_, _, _) => 4,
      Self::Store(_) => 2,
      Self::Print(_) => 2,
      Self::JumpIfTrue(_, _) => 0,
      Self::JumpIfFalse(_, _) => 0,
      Self::LessThan(_, _, _) => 4,
      Self::Equals(_, _, _) => 4,
    }
  }

  pub fn parse(
    opcode: i32,
    operand_one: i32,
    operand_two: i32,
    dest: usize,
    input: &Vec<i32>,
  ) -> Self {
    let opcode_digits = Self::opcode_digits(opcode);
    Self::parse_with_opcode_digits(opcode_digits, operand_one, operand_two, dest, input)
  }

  pub fn parse_with_opcode_digits(
    opcode: Vec<i32>,
    operand_one: i32,
    operand_two: i32,
    dest: usize,
    input: &Vec<i32>,
  ) -> Self {
    if opcode[0] >= 1 && opcode[0] <= 8 {
      let value_operand_one = Self::operand_value(opcode[2], operand_one, input);
      let value_operand_two = Self::operand_value(opcode[3], operand_two, input);
      match opcode[0] {
        1 => Self::Sum(value_operand_one, value_operand_two, dest),
        2 => Self::Multiply(value_operand_one, value_operand_two, dest),
        3 => Self::Store(operand_one as usize),
        4 => Self::Print(value_operand_one),
        5 => Self::JumpIfTrue(value_operand_one, value_operand_two as usize),
        6 => Self::JumpIfFalse(value_operand_one, value_operand_two as usize),
        7 => Self::LessThan(value_operand_one, value_operand_two, dest),
        8 => Self::Equals(value_operand_one, value_operand_two, dest),
        _ => unreachable!(),
      }
    } else {
      unreachable!()
    }
  }

  pub fn opcode_digits(opcode: i32) -> Vec<i32> {
    let opcode_with_insignifficant_zeros = format!("{:<05}", opcode);
    let mut instruction = digits(opcode_with_insignifficant_zeros);
    instruction.reverse();
    instruction
  }

  fn operand_value(mode: i32, operand: i32, input: &Vec<i32>) -> i32 {
    if mode == 0 {
      *input.get(operand as usize).unwrap_or(&-999_999)
    } else {
      operand
    }
  }
}

#[cfg(tests)]

mod tests {
  #[test]
  fn test_operand_value() {}
}
