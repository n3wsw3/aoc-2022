use std::fmt;

use itertools::Itertools;

type Coord = (i32, i32);

#[derive(Debug)]
struct Line(Vec<Coord>);

impl Line {
  fn from_str(str: &str) -> Line {
    Line(
      str
        .split(" -> ")
        .map(|coord| {
          coord
            .split(',')
            .map(|pos| pos.parse().unwrap())
            .collect_tuple()
            .unwrap()
        })
        .collect_vec(),
    )
  }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Fill {
  Air,
  Sand,
  Rock,
}

impl fmt::Display for Fill {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Fill::Air => ' ',
        Fill::Rock => '#',
        Fill::Sand => 'o',
      }
    )
  }
}

#[derive(Debug)]
struct Map {
  map: Vec<Vec<Fill>>,
  start: Coord,
}

impl fmt::Display for Map {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in 0..self.map[0].len() {
      for x in 0..self.map.len() {
        write!(f, "{}", self.map[x][y]).ok();
      }
      writeln!(f).ok();
    }
    write!(f, "")
  }
}

impl Map {
  fn from_str(str: &str) -> Map {
    let lines = str.lines().map(Line::from_str).collect_vec();

    let max_y = lines
      .iter()
      .map(|line| line.0.iter().map(|(_, y)| y).max().unwrap())
      .max()
      .unwrap();

    let size_x = (max_y + 2) * 2 + 1;
    let offset_x = -(size_x / 2 - 500);
    let size_y = max_y + 3;

    let mut map = vec![vec![Fill::Air; size_y as usize]; size_x as usize];

    lines
      .iter()
      .map(|line| {
        line
          .0
          .iter()
          .copied()
          .tuple_windows::<(Coord, Coord)>()
          .map(|((x1, y1), (x2, y2))| {
            if x1 == x2 {
              (i32::min(y1, y2)..i32::max(y1, y2) + 1)
                .map(|ys| (x1, ys))
                .collect_vec()
            } else {
              (i32::min(x1, x2)..i32::max(x1, x2) + 1)
                .map(|xs| (xs, y1))
                .collect_vec()
            }
          })
          .concat()
      })
      .concat()
      .iter()
      .for_each(|(x, y)| map[(*x - offset_x) as usize][*y as usize] = Fill::Rock);

    (0..size_x)
      .map(|xs| (xs, size_y - 1))
      .for_each(|(x, y)| map[x as usize][y as usize] = Fill::Rock);

    Map {
      map,
      start: (500 - offset_x, 0),
    }
  }

  fn place_sand(&mut self) {
    fn offset<A: std::ops::Add<Output = A>>((x, y): (A, A), (dx, dy): (A, A)) -> (A, A) {
      (x + dx, y + dy)
    }

    fn is_air(map: &Map, coord: Coord) -> bool {
      map.map.len() > coord.0 as usize
        && map.map[0].len() > coord.1 as usize
        && map.map[coord.0 as usize][coord.1 as usize] == Fill::Air
    }
    let movements: [(i32, i32); 3] = [(0, 1), (-1, 1), (1, 1)];

    let mut pos = self.start;

    while let Some(new_pos) = movements
      .map(|m| offset(m, pos))
      .iter()
      .find(|&&coord| is_air(self, coord))
    {
      pos = *new_pos;
    }

    self.map[pos.0 as usize][pos.1 as usize] = Fill::Sand;
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  let mut map = Map::from_str(input);

  let mut count = 0;

  while map
    .map
    .iter()
    .map(|v| {
      v.iter()
        .enumerate()
        .filter(|(_, f)| **f == Fill::Sand)
        .map(|(i, _)| i)
        .collect_vec()
    })
    .concat()
    .iter()
    .filter(|y| **y >= 9)
    .collect_vec()
    .is_empty()
  {
    map.place_sand();
    count += 1;
  }

  Some(count - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut map = Map::from_str(input);

  let mut count = 0;

  while map.map[map.start.0 as usize][map.start.1 as usize] != Fill::Sand {
    map.place_sand();
    count += 1;
  }

  Some(count)
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 14);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 14);
    assert_eq!(part_one(&input), Some(24));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 14);
    assert_eq!(part_two(&input), Some(93));
  }
}
