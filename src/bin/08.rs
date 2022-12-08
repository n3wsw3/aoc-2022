use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<u32>> {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| (c as u32) - ('0' as u32))
        .collect_vec()
    })
    .collect_vec()
}

fn height_at(pos: (usize, usize), map: &[Vec<u32>]) -> Option<&u32> {
  match map.get(pos.0) {
    Some(row) => row.get(pos.1),
    None => None,
  }
}

fn visible(map: &Vec<Vec<u32>>) -> impl Fn((usize, usize)) -> bool + '_ {
  move |(x, y)| {
    let size = map.len();

    [
      (0..y).map(|v| (x, v)).collect(),
      (y + 1..size).map(|v| (x, v)).collect(),
      (0..x).map(|v| (v, y)).collect(),
      (x + 1..size).map(|v| (v, y)).collect(),
    ]
    .iter()
    .map(|coords: &Vec<(usize, usize)>| {
      coords
        .iter()
        .all(|pos| height_at(*pos, map).unwrap() < height_at((x, y), map).unwrap())
    })
    .any(|b| b)
  }
}

pub fn part_one(input: &str) -> Option<usize> {
  let map = parse(input);
  Some(
    GridIter::new(map.len(), map.len())
      .map(visible(&map))
      .filter(|v| *v)
      .count(),
  )
}

fn score(map: &[Vec<u32>]) -> impl Fn((usize, usize)) -> u32 + '_ {
  move |(x, y)| {
    let height = height_at((x, y), map).unwrap();
    let mut score: u32 = 1;

    for (offset_x, offset_y) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
      let mut last_pos = (x, y);
      let mut num = 0;
      while let Some(h) = height_at(
        (
          (last_pos.0 as i32 + offset_x) as usize,
          (last_pos.1 as i32 + offset_y) as usize,
        ),
        map,
      ) {
        last_pos = (
          (last_pos.0 as i32 + offset_x) as usize,
          (last_pos.1 as i32 + offset_y) as usize,
        );
        num += 1;
        if h >= height {
          break;
        }
      }
      score *= num;
    }

    score
  }
}

pub fn part_two(input: &str) -> Option<u32> {
  let map = parse(input);

  GridIter::new(map.len(), map.len()).map(score(&map)).max()
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 8);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 8);
    assert_eq!(part_one(&input), Some(21));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 8);
    assert_eq!(part_two(&input), Some(8));
  }
}

pub struct GridIter {
  cur_x: usize,
  cur_y: usize,
  max_x: usize,
  max_y: usize,
  first: bool,
}

impl GridIter {
  fn new(max_x: usize, max_y: usize) -> GridIter {
    GridIter {
      cur_x: 0,
      cur_y: 0,
      max_x,
      max_y,
      first: true,
    }
  }
}

impl Iterator for GridIter {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    if self.first {
      self.first = false;
      return Some((self.cur_x, self.cur_y));
    }
    self.cur_x += 1;
    if self.cur_x >= self.max_x {
      self.cur_x %= self.max_x;
      self.cur_y += 1;
    }
    if self.cur_y >= self.max_y {
      None
    } else {
      Some((self.cur_x, self.cur_y))
    }
  }
}
