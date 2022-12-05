use itertools::Itertools;

fn parse_sections(value: &str) -> (u32, u32) {
  value
    .split("-")
    .map(|val| val.parse::<u32>().unwrap())
    .collect_tuple()
    .unwrap()
}

fn parse_pairs(input: &str) -> Vec<((u32, u32), (u32, u32))> {
  input
    .split("\n")
    .into_iter()
    .map(|line| line.split(",").collect_vec())
    .map(|x| {
      x.into_iter()
        .map(|y| parse_sections(y))
        .collect_tuple::<(_, _)>()
        .unwrap()
    })
    .collect_vec()
}

fn section_contains((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> bool {
  (end1 >= end2 && start1 <= start2) || (end2 >= end1 && start2 <= start1)
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    parse_pairs(input)
      .into_iter()
      .map(|(first, last)| if section_contains(first, last) { 1 } else { 0 })
      .sum(),
  )
}

fn section_contains2((start1, end1): (u32, u32), (start2, end2): (u32, u32)) -> bool {
  (start2 <= start1 && end2 >= start1) || (start1 <= start2 && end1 >= start2)
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    parse_pairs(input)
      .into_iter()
      .map(|(first, last)| if section_contains2(first, last) { 1 } else { 0 })
      .sum(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 4);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 4);
    assert_eq!(part_one(&input), Some(2));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 4);
    assert_eq!(part_two(&input), Some(4));
  }
}
