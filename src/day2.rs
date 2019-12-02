#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
  input
    .split(",")
    .map(|l| l.trim().parse::<usize>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<usize>) -> usize {
  solve_with_params(input, 12, 2)
}

pub fn solve_with_params(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
  let mut opcodes = input.clone();
  opcodes[1] = noun;
  opcodes[2] = verb;
  let mut index = 0;
  loop {
    if let [opcode, operand_one, operand_two, dest] = opcodes[index..index + 4] {
      match opcode {
        99 => return opcodes[0],
        1 => opcodes[dest] = opcodes[operand_one] + opcodes[operand_two],
        2 => opcodes[dest] = opcodes[operand_one] * opcodes[operand_two],
        _ => panic!(
          "Error: Unexpected number ({}), first number is: {}",
          opcode, opcodes[0]
        ),
      }
      index += 4;
    }
  }
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<usize>) -> usize {
  for noun in 0..99 {
    for verb in 0..99 {
      if solve_with_params(input, noun, verb) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("It did not find a solution");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn generator_test() {
    let input = "1,9,10,3,
    2,3,11,0,
    99,
    30,40,50";

    let result = input_generator(input);
    assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], result);
  }
}
