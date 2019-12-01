use rayon::prelude::*;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|l| l.trim().parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
  input.iter().fold(0, |acc, mass| {
    let fuel = calculate_base_fuel_for(mass);
    acc + fuel
  })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
  input.iter().fold(0, |acc, mass| {
    let fuel = calculate_fuel_for(mass);
    acc + fuel
  })
}

#[aoc(day1, part2, Parallel)]
pub fn solve_part2_in_parallel(input: &Vec<i32>) -> i32 {
  input.par_iter().map(|mass| calculate_fuel_for(mass)).sum()
}

pub fn calculate_base_fuel_for(mass: &i32) -> i32 {
  (mass / 3) - 2
}

pub fn calculate_fuel_for(mass: &i32) -> i32 {
  let base_fuel = calculate_base_fuel_for(mass);
  if base_fuel < 0 {
    return 0;
  }
  base_fuel + calculate_fuel_for(&base_fuel)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn input_generator_test() {
    let input = "1\n3\n4\n";
    let parsed_input = input_generator(input);
    assert_eq!(parsed_input[0], 1);
    assert_eq!(parsed_input[1], 3);
    assert_eq!(parsed_input[2], 4);
    assert_eq!(parsed_input.len(), 3);
  }

  #[test]
  fn calculate_base_fuel_for_test() {
    assert_eq!(calculate_base_fuel_for(&12), 2);
    assert_eq!(calculate_base_fuel_for(&14), 2);
    assert_eq!(calculate_base_fuel_for(&1969), 654);
    assert_eq!(calculate_base_fuel_for(&100756), 33583);
  }

  #[test]
  fn calculate_fuel_for_test() {
    assert_eq!(calculate_fuel_for(&12), 2);
    assert_eq!(calculate_fuel_for(&14), 2);
    assert_eq!(calculate_fuel_for(&1969), 966);
    assert_eq!(calculate_fuel_for(&100756), 50346);
  }

  #[test]
  fn solve_part1_test() {
    let input = vec![12, 14, 1969, 100756];
    assert_eq!(solve_part1(&input), 2 + 2 + 654 + 33583);
  }

  #[test]
  fn solve_part2_test() {
    let input = vec![12, 14, 1969, 100756];
    assert_eq!(solve_part2(&input), 2 + 2 + 966 + 50346);
  }
}
