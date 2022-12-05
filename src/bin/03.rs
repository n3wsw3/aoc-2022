use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    input
      .split("\n")
      .map(|line| {
        let (first, last) = line.split_at(line.len() / 2);

        // Remove duplicate characters in first
        let x = first.chars().collect::<HashSet<char>>();
        let y = last.chars().collect::<HashSet<char>>();
        // Find the intersection of the two sets
        let z = x.intersection(&y).collect::<Vec<&char>>();

        // x.get(0) to u32 a-z = 1-26 and A-Z = 27-52
        z.get(0)
          .map(|c| {
            if c.is_lowercase() {
              (c.to_ascii_uppercase() as u32) - 64
            } else {
              (c.to_ascii_uppercase() as u32) - 38
            }
          })
          .unwrap_or(0)
      })
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  // combine every three lines from input.split("\n")
  Some(
    input
      .split("\n")
      .collect::<Vec<&str>>()
      .chunks(3)
      .map(|chunk| {
        let set = chunk
          .into_iter()
          .map(|backpacks| backpacks.chars().collect::<HashSet<char>>())
          .reduce(|acc, hs| {
            let x = acc.intersection(&hs).cloned().collect();
            x
          })
          .unwrap()
          .into_iter()
          .collect::<Vec<char>>();
        set
          .get(0)
          .map(|c| {
            if c.is_lowercase() {
              (c.to_ascii_uppercase() as u32) - 64
            } else {
              (c.to_ascii_uppercase() as u32) - 38
            }
          })
          .unwrap_or(0)
      })
      .sum(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 3);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 3);
    assert_eq!(part_one(&input), Some(157));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 3);
    assert_eq!(part_two(&input), Some(70));
  }
}
