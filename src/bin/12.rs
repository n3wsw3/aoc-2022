use std::collections::VecDeque;

#[derive(Debug)]
struct ElevationMap {
  heights: Vec<Vec<u32>>,
  start: (usize, usize),
  end: (usize, usize),
}

trait AsNumber {
  fn as_number(&self) -> u32;
}

impl AsNumber for char {
  fn as_number(&self) -> u32 {
    *self as u32 - 'a' as u32
  }
}

fn parse(input: &str) -> (ElevationMap, usize, usize) {
  let mut elevations = ElevationMap {
    start: (0, 0),
    end: (0, 0),
    heights: Vec::new(),
  };
  for (index, line) in input.lines().enumerate() {
    let mut row = Vec::new();
    for (jndex, c) in line.chars().enumerate() {
      match c {
        'S' => {
          elevations.start = (index, jndex);
          row.push('a'.as_number());
        }
        'E' => {
          elevations.end = (index, jndex);
          row.push('z'.as_number());
        }
        _ => {
          row.push(c.as_number());
        }
      }
    }
    elevations.heights.push(row);
  }

  let size_x = elevations.heights.len();
  let size_y = elevations.heights[0].len();

  (elevations, size_x, size_y)
}

fn search_until_condition<F>(
  (elevations, size_x, size_y): (ElevationMap, usize, usize),
  condition: F,
) -> Option<u32>
where
  F: Fn((usize, usize), &ElevationMap) -> bool,
{
  let neighbours = |i: usize, j: usize| -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if i > 0 {
      n.push((i - 1, j));
    }
    if j > 0 {
      n.push((i, j - 1));
    }
    if i < size_x - 1 {
      n.push((i + 1, j));
    }
    if j < size_y - 1 {
      n.push((i, j + 1));
    }
    n
  };

  // Steps needed to reach each position
  let mut steps_to_pos = vec![vec![None; size_y]; size_x];
  steps_to_pos[elevations.end.0][elevations.end.1] = Some(0);

  let mut positions = VecDeque::new();
  positions.push_back(elevations.end);

  while let Some((i, j)) = positions.pop_front() {
    if condition((i, j), &elevations) {
      return steps_to_pos[i][j];
    }
    let current_steps = steps_to_pos[i][j].unwrap_or(u32::MAX - 1);
    let current_elevation = elevations.heights[i][j] as i32;

    for (new_i, new_j) in neighbours(i, j) {
      let next_elevation = elevations.heights[new_i][new_j] as i32;

      if steps_to_pos[new_i][new_j].is_none() && current_elevation - next_elevation <= 1
      {
        steps_to_pos[new_i][new_j] = Some(current_steps + 1);
        positions.push_back((new_i, new_j));
      }
    }
  }
  None
}

pub fn part_one(input: &str) -> Option<u32> {
  search_until_condition(parse(input), |pos, map| pos == map.start)
}

pub fn part_two(input: &str) -> Option<u32> {
  search_until_condition(parse(input), |(i, j), map| map.heights[i][j] == 0)
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 12);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 12);
    assert_eq!(part_one(&input), Some(31));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 12);
    assert_eq!(part_two(&input), Some(29));
  }
}
