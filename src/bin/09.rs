use itertools::Itertools;

type Pos = (i32, i32);

fn parse(input: &str) -> Vec<char> {
  input
    .lines()
    .map(|line| {
      let (dir, length) = line.split(' ').collect_tuple().unwrap();
      vec![dir.chars().next().unwrap(); length.parse().unwrap()]
    })
    .concat()
}

fn solve<A, B, C: PartialEq + std::cmp::Ord, F>(
  instructions: Vec<char>,
  init: (A, B, Vec<C>),
  f: F,
) -> usize
where
  F: FnMut((A, B, Vec<C>), &char) -> (A, B, Vec<C>),
{
  instructions
    .iter()
    .fold(init, f)
    .2
    .iter()
    .sorted()
    .dedup()
    .collect_vec()
    .len()
}

fn move_tail((head_x, head_y): Pos, (tail_x, tail_y): Pos) -> Pos {
  let (diff_x, diff_y) = (head_x - tail_x, head_y - tail_y);
  if i32::abs(diff_x) >= 2 && i32::abs(diff_y) >= 2 {
    return (
      head_x - diff_x / i32::abs(diff_x),
      head_y - diff_y / i32::abs(diff_y),
    );
  } else if i32::abs(diff_x) >= 2 {
    return (head_x - diff_x / i32::abs(diff_x), head_y);
  } else if i32::abs(diff_y) >= 2 {
    return (head_x, head_y - diff_y / i32::abs(diff_y));
  }
  (tail_x, tail_y)
}

fn move_head((pos_x, pos_y): Pos, dir: char) -> Pos {
  match dir {
    'U' => (pos_x, pos_y + 1),
    'D' => (pos_x, pos_y - 1),
    'L' => (pos_x - 1, pos_y),
    'R' => (pos_x + 1, pos_y),
    _ => (pos_x, pos_y),
  }
}

pub fn part_one(input: &str) -> Option<usize> {
  Some(solve(
    parse(input),
    ((0, 0), (0, 0), Vec::new()),
    |(head, tail, mut prev), dir| {
      let head_pos = move_head(head, *dir);
      let tail_pos = move_tail(head_pos, tail);
      prev.push(tail);
      (head_pos, tail_pos, prev)
    },
  ))
}

pub fn part_two(input: &str) -> Option<usize> {
  Some(solve(
    parse(input),
    ((0, 0), vec![(0, 0); 9], Vec::new()),
    |(head, tail, mut prev), dir| {
      let head_pos = move_head(head, *dir);
      let mut new_pieces: Vec<Pos> = vec![];
      for piece in tail {
        new_pieces.push(move_tail(*new_pieces.last().unwrap_or(&head_pos), piece));
      }
      let last = new_pieces.last().unwrap();
      prev.push(*last);
      (head_pos, new_pieces, prev)
    },
  ))
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 9);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 9);
    assert_eq!(part_one(&input), Some(87));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 9);
    assert_eq!(part_two(&input), Some(36));
  }
}
