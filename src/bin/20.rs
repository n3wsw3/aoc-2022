use itertools::Itertools;

fn mix(vec: &mut Vec<(usize, i64)>) {
  let mut to_move = 0;

  while to_move < vec.len() {
    let index = vec.iter().position(|(index, _)| *index == to_move).unwrap();
    let (_, val) = vec[index];
    let mut new_index = (index as i64 + val) % (vec.len() as i64 - 1);

    if new_index.is_negative() {
      new_index += vec.len() as i64 - 1;
    }

    vec.remove(index);
    vec.insert(new_index as usize, (to_move, val));
    to_move += 1;
  }
}

pub fn part_one(input: &str) -> Option<i64> {
  let mut x = input
    .lines()
    .map(|line| line.parse::<i64>().unwrap())
    .enumerate()
    .collect_vec();

  mix(&mut x);

  let index_zero = x.iter().position(|(_, val)| *val == 0).unwrap();

  Some(
    [
      x[(index_zero + 1000) % x.len()],
      x[(index_zero + 2000) % x.len()],
      x[(index_zero + 3000) % x.len()],
    ]
    .iter()
    .map(|(_, v)| *v)
    .sum(),
  )
}

pub fn part_two(input: &str) -> Option<i64> {
  let mut x = input
    .lines()
    .map(|line| line.parse::<i64>().unwrap() * 811589153)
    .enumerate()
    .collect_vec();

  for _ in 0..10 {
    mix(&mut x);
  }

  let index_zero = x.iter().position(|(_, val)| *val == 0).unwrap();

  Some(
    [
      x[(index_zero + 1000) % x.len()],
      x[(index_zero + 2000) % x.len()],
      x[(index_zero + 3000) % x.len()],
    ]
    .iter()
    .map(|(_, v)| *v)
    .sum(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 20);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 20);
    assert_eq!(part_one(&input), Some(3));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 20);
    assert_eq!(part_two(&input), None);
  }
}
