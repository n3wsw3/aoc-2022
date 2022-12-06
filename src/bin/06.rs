use itertools::Itertools;
use std::collections::HashSet;

fn solution(input: &str, window_size: usize) -> usize {
  input
    .as_bytes()
    .windows(window_size)
    .map(|x| x.iter().collect::<HashSet<_>>().len())
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
    input.split('\n').map(|line| line.split(' ').collect_tuple().unwrap()).for_each(|(i, answer, _)| assert_eq!(part_one(i), answer.parse().ok()));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 6);
    input.split('\n').map(|line| line.split(' ').collect_tuple().unwrap()).for_each(|(i, _, answer)| assert_eq!(part_two(i), answer.parse().ok()));
  }
}
