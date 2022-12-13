use std::{cmp::Ordering, str::Chars};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
  List(Vec<Node>),
  Num(u32),
}

impl Node {
  fn from_str(str: &str) -> Node {
    fn recursive(input: &mut Chars) -> Node {
      let mut result = Vec::new();
      while let Some(thing) = input.next() {
        match thing {
          '[' => {
            result.push(recursive(input));
          }
          ']' => {
            break;
          }
          ',' => {}
          x => {
            let mut val = (x as usize - '0' as usize) as u32;
            while let Some(v) = input.peekable().peek().unwrap().as_number() {
              val *= 10;
              val += v;
            }
            result.push(Node::Num(val));
          }
        }
      }
      Node::List(result)
    }
    recursive(&mut str.chars())
  }
}

trait AsNum {
  fn as_number(&self) -> Option<u32>;
}

impl AsNum for char {
  fn as_number(&self) -> Option<u32> {
    let val = *self as i32 - '0' as i32;
    if (0..10).contains(&val) {
      Some(val as u32)
    } else {
      None
    }
  }
}

impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Node::Num(x), Node::Num(y)) => x.partial_cmp(y),
      (Node::List(x), Node::List(y)) => x.partial_cmp(y),
      (Node::Num(_), Node::List(_)) => Node::List(vec![self.clone()]).partial_cmp(other),
      (Node::List(_), Node::Num(_)) => self.partial_cmp(&Node::List(vec![other.clone()])),
    }
  }
}

impl Ord for Node {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    input
      .split("\n\n")
      .map(|l| l.lines().map(Node::from_str).collect_tuple().unwrap())
      .enumerate()
      .fold(
        0,
        |acc, (i, (p1, p2))| if p1 < p2 { acc + i as u32 + 1 } else { acc },
      ),
  )
}

pub fn part_two(input: &str) -> Option<usize> {
  Some(
    vec![input.lines().collect(), vec!["[[2]]", "[[6]]"]]
      .concat()
      .iter()
      .filter(|l| !l.is_empty())
      .map(|l| Node::from_str(l))
      .sorted()
      .enumerate()
      .filter(|(_, packet)| {
        *packet == Node::from_str("[[2]]") || *packet == Node::from_str("[[6]]")
      })
      .map(|(i, _)| i + 1)
      .product(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 13);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 13);
    assert_eq!(part_one(&input), Some(13));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 13);
    assert_eq!(part_two(&input), Some(140));
  }
}
