use std::collections::{HashMap, HashSet};
use std::iter;
use std::num::ParseIntError;
use std::str::FromStr;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {
  input
    .split("\n")
    .map(|l| l.trim().parse::<Wire>().unwrap())
    .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Wire>) -> u64 {
  let first_wire = input.first().unwrap();
  let second_wire = input.last().unwrap();

  let first_wire_positions: HashSet<Position> = first_wire.positions().collect();
  second_wire
    .positions()
    .filter_map(|position| {
      if first_wire_positions.contains(&position) {
        Some(position.manhatthan_distance())
      } else {
        None
      }
    })
    .min()
    .unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Wire>) -> u64 {
  let wire_one = input.first().unwrap();
  let wire_two = input.last().unwrap();

  let wire_one_positions: HashMap<Position, u64> = wire_one.positions().zip(1..).collect();
  wire_two
    .positions()
    .zip(1..)
    .filter_map(|(position, steps_two)| {
      wire_one_positions
        .get(&position)
        .map(|steps_one| steps_one + steps_two)
    })
    .min()
    .unwrap()
}
#[derive(Clone, Debug, PartialEq, Default, Hash, Eq)]
pub struct Position {
  pub x: i64,
  pub y: i64,
}

impl Position {
  pub fn move_in(&mut self, moving_direction: Direction) {
    match moving_direction {
      Direction::Down => self.x += 1,
      Direction::Up => self.x -= 1,
      Direction::Left => self.y -= 1,
      Direction::Right => self.y += 1,
    };
  }

  pub fn manhatthan_distance(&self) -> u64 {
    (self.x.abs() + self.y.abs()) as u64
  }
}
#[derive(Clone)]
pub struct Wire {
  pub segments: Vec<WireSegment>,
}

impl FromStr for Wire {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let segments: Vec<WireSegment> = s
      .split(",")
      .map(|l| l.trim().parse::<WireSegment>().unwrap())
      .collect();
    Ok(Self { segments })
  }
}

impl Wire {
  pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
    self
      .segments
      .iter()
      .flat_map(|segment| iter::repeat(segment.direction).take(segment.distance))
      .scan(Position::default(), |current_position, direction| {
        current_position.move_in(direction);
        Some(current_position.clone())
      })
  }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
  Up,
  Down,
  Right,
  Left,
}

impl FromStr for Direction {
  type Err = ParseIntError;

  fn from_str(direction_str: &str) -> Result<Self, Self::Err> {
    match direction_str {
      "U" => Ok(Direction::Up),
      "D" => Ok(Direction::Down),
      "R" => Ok(Direction::Right),
      "L" => Ok(Direction::Left),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WireSegment {
  pub direction: Direction,
  pub distance: usize,
}

impl FromStr for WireSegment {
  type Err = ParseIntError;

  fn from_str(segment: &str) -> Result<Self, Self::Err> {
    let (direction_str, distance_str) = segment.split_at(1);
    let distance = distance_str.parse().unwrap();
    let direction = direction_str.parse().unwrap();

    Ok(WireSegment {
      direction,
      distance,
    })
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]

  fn test_solve_part2() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83";
    let segments = input_generator(input);

    let result = solve_part2(&segments);
    assert_eq!(result, 610);
  }

  #[test]

  fn test_solve_part1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83";
    let segments = input_generator(input);

    let result = solve_part1(&segments);
    assert_eq!(result, 159);
  }

  #[test]
  fn test_input_generator() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83";
    let wires = input_generator(input);

    let first_wire = wires.first().unwrap();

    assert_eq!(first_wire.segments.len(), 9);

    let first_segment_from_first_wire = first_wire.segments.first().unwrap();
    assert_eq!(first_segment_from_first_wire.direction, Direction::Right);
    assert_eq!(first_segment_from_first_wire.distance, 75);

    let second_wire = wires.last().unwrap();

    let first_segment_from_second_wire = second_wire.segments.first().unwrap();
    assert_eq!(first_segment_from_second_wire.direction, Direction::Up);
    assert_eq!(first_segment_from_second_wire.distance, 62);
  }
}
