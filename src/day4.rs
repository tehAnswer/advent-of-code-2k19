use std::collections::HashMap;
use std::ops::Range;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Range<u32> {
  let vector: Vec<u32> = input
    .split("-")
    .map(|n| n.parse::<u32>().unwrap())
    .collect();
  (vector[0]..vector[1] + 1)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Range<u32>) -> usize {
  let passwords = find_valid_passwords_in(input, is_valid_password_in_part1);
  passwords.len()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Range<u32>) -> usize {
  let passwords = find_valid_passwords_in(input, is_valid_password_in_part2);
  passwords.len()
}

pub fn find_valid_passwords_in(range: &Range<u32>, validator: fn(u32) -> bool) -> Vec<u32> {
  let mut valid_passwords = Vec::new();
  for password in range.clone() {
    if validator(password) {
      valid_passwords.push(password)
    }
  }
  valid_passwords
}

#[derive(Default)]
struct Validations {
  pub increasing: bool,
  pub double_count: HashMap<u32, u32>,
}

pub fn is_valid_password_in_part1(password: u32) -> bool {
  let validations = scan_password(password);
  validations.increasing && validations.double_count.len() >= 1
}

pub fn is_valid_password_in_part2(password: u32) -> bool {
  let validations = scan_password(password);
  validations.increasing
    && validations.double_count.len() > 0
    && validations.double_count.values().any(|count| *count == 1)
}

fn scan_password(password: u32) -> Validations {
  let numbers: Vec<u32> = password
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u32)
    .collect();

  numbers.iter().enumerate().fold(
    Validations {
      increasing: true,
      ..Validations::default()
    },
    |acc, (index, number)| {
      if let Some(next_value) = numbers.get(index + 1) {
        let increasing = acc.increasing && number <= next_value;
        let mut double_count = acc.double_count.clone();
        if number == next_value {
          double_count.insert(number.clone(), double_count.get(number).unwrap_or(&0) + 1);
        }
        Validations {
          increasing,
          double_count,
        }
      } else {
        acc
      }
    },
  )
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_valid_password_in_part2() {
    assert_eq!(is_valid_password_in_part2(112233), true);
    assert_eq!(is_valid_password_in_part2(112256), true);
    assert_eq!(is_valid_password_in_part2(122356), true);
    assert_eq!(is_valid_password_in_part2(123456), false);
    assert_eq!(is_valid_password_in_part2(111134), false);
  }

  #[test]
  fn test_is_valid_password_in_part1() {
    assert_eq!(is_valid_password_in_part1(112233), true);
    assert_eq!(is_valid_password_in_part1(112256), true);
    assert_eq!(is_valid_password_in_part1(122356), true);
    assert_eq!(is_valid_password_in_part1(123456), false);
    assert_eq!(is_valid_password_in_part1(111134), true);
  }
}
