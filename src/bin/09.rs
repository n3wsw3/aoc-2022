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

fn new_pos((head_x, head_y): Pos, (tail_x, tail_y): Pos) -> Pos {
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

fn move_pos((pos_x, pos_y): Pos, dir: char) -> Pos {
  match dir {
    'U' => (pos_x, pos_y + 1),
    'D' => (pos_x, pos_y - 1),
    'L' => (pos_x - 1, pos_y),
    'R' => (pos_x + 1, pos_y),
    _ => (pos_x, pos_y),
  }
}

pub fn part_one(input: &str) -> Option<usize> {
  let mut x = parse(input)
    .iter()
    .fold(
      ((0, 0), (0, 0), Vec::new()),
      |(head, tail, mut prev), dir| {
        let head_pos = move_pos(head, *dir);
        let tail_pos = new_pos(head_pos, tail);
        prev.push(tail);
        (head_pos, tail_pos, prev)
      },
    )
    .2;
  x.sort();
  x.dedup();
  Some(x.len())
}

pub fn part_two(input: &str) -> Option<usize> {
  let mut x = parse(input)
    .iter()
    .fold(
      ((0, 0), vec![(0, 0); 9], Vec::new()),
      |(head, tail, mut prev), dir| {
        let head_pos = move_pos(head, *dir);
        let mut new_pieces: Vec<Pos> = vec![];
        for piece in tail {
          new_pieces.push(new_pos(*new_pieces.last().unwrap_or(&head_pos), piece));
        }
        let last = new_pieces.last().unwrap();
        prev.push(*last);
        (head_pos, new_pieces, prev)
      },
    )
    .2;
  x.sort();
  x.dedup();
  Some(x.len())
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
