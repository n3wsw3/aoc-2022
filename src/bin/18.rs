use itertools::Itertools;
use std::collections::VecDeque;

type Pos = (i32, i32, i32);

fn add(pos: Pos, other: Pos) -> Pos {
  (pos.0 + other.0, pos.1 + other.1, pos.2 + other.2)
}

fn offsets(pos: &Pos) -> Vec<Pos> {
  vec![
    (0, 0, 1),
    (0, 1, 0),
    (1, 0, 0),
    (0, 0, -1),
    (0, -1, 0),
    (-1, 0, 0),
  ]
  .iter()
  .map(|off| (add(*off, *pos)))
  .collect_vec()
}

fn parse(input: &str) -> (Pos, Pos, Vec<Pos>) {
  let mut min = (0, 0, 0);
  let mut max = (0, 0, 0);

  let blocks = input
    .lines()
    .map(|line| {
      let (x, y, z) = line
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect_tuple()
        .unwrap();

      if x < min.0 {
        min.0 = x
      } else if x > max.0 {
        max.0 = x
      }

      if y < min.1 {
        min.1 = y
      } else if y > max.1 {
        max.1 = y
      }

      if z < min.2 {
        min.2 = z
      } else if z > max.2 {
        max.2 = z
      }

      (x, y, z)
    })
    .collect_vec();

  (min, max, blocks)
}

pub fn part_one(input: &str) -> Option<u32> {
  let (_, _, blocks) = parse(input);

  Some(
    blocks
      .iter()
      .map(|block| {
        offsets(block)
          .iter()
          .fold(0, |acc, off| acc + u32::from(!blocks.contains(off)))
      })
      .sum::<u32>(),
  )
}

fn within_bounds(min: Pos, max: Pos, pos: Pos) -> bool {
  max.0 >= pos.0
    && pos.0 >= min.0
    && max.1 >= pos.1
    && pos.1 >= min.1
    && max.2 >= pos.2
    && pos.2 >= min.2
}

pub fn part_two(input: &str) -> Option<u32> {
  let (min, max, blocks) = parse(input);

  let mut visited: Vec<Pos> = Vec::new();
  let min = (min.0 - 1, min.1 - 1, min.2 - 1);
  let max = (max.0 + 1, max.1 + 1, max.2 + 1);

  let mut positions = VecDeque::new();
  positions.push_front(max);

  let mut external_surface = 0;

  while let Some(pos) = positions.pop_front() {
    for new_pos in offsets(&pos) {
      if blocks.contains(&new_pos) {
        external_surface += 1;
      } else if within_bounds(min, max, new_pos)
        && !positions.contains(&new_pos)
        && !visited.contains(&new_pos)
      {
        positions.push_back(new_pos);
      }
    }

    visited.push(pos);
  }

  Some(external_surface)
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 18);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 18);
    assert_eq!(part_one(&input), Some(64));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 18);
    assert_eq!(part_two(&input), Some(58));
  }
}
