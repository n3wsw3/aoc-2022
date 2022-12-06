use std::collections::HashSet;
use itertools::Itertools;

fn solution(input: &str, window_size: usize) -> usize {
  input
    .as_bytes()
    .windows(window_size)
    .map(|x| x.into_iter().collect::<HashSet<_>>().len())
    .find_position(|x| *x == window_size)
    .unwrap()
    .0
    + window_size
}

pub fn part_one(input: &str) -> Option<usize> {
  Some(solution(input, 4))
}

pub fn part_two(input: &str) -> Option<usize> {
  Some(solution(input, 14))
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 6);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 6);
    assert_eq!(part_one(&input), Some(5));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 6);
    assert_eq!(part_two(&input), Some(23));
  }
}
