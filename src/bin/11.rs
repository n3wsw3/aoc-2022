use itertools::Itertools;
use std::collections::VecDeque;

enum Op {
  Add(u64),
  Multiply(u64),
  Square,
}

struct Monkey {
  items: VecDeque<u64>,
  op: Op,
  test: u64,
  throw_to: Vec<usize>,
  items_handeled: u64,
}

impl Monkey {
  fn from_str(lines: &str) -> Monkey {
    let lines = lines.lines().map(|l| l.trim()).collect_vec();
    let mut op = Op::Add(0);
    let mut test = 1;
    let mut truth = 1;
    let mut untruth = 1;
    let mut items = VecDeque::new();
    for line in lines {
      match *line.split(' ').collect_vec() {
        ["Monkey", _] => {}
        ["Operation:", "new", "=", "old", "*", "old"] => op = Op::Square,
        ["Operation:", "new", "=", "old", op_type, val] => match op_type {
          "*" => op = Op::Multiply(val.parse().unwrap()),
          _ => op = Op::Add(val.parse().unwrap()),
        },
        ["Test:", "divisible", "by", test_str] => test = test_str.parse().unwrap(),
        ["If", "true:", "throw", "to", "monkey", monkey_id] => truth = monkey_id.parse().unwrap(),
        ["If", "false:", "throw", "to", "monkey", monkey_id] => {
          untruth = monkey_id.parse().unwrap()
        }
        _ => {
          items = line[16..]
            .split(", ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect()
        }
      }
    }
    Monkey {
      items,
      op,
      test,
      throw_to: vec![untruth, truth],
      items_handeled: 0,
    }
  }
  fn apply_op(&self, item: u64) -> u64 {
    match self.op {
      Op::Multiply(v) => item * v,
      Op::Add(v) => item + v,
      Op::Square => item * item,
    }
  }
}

fn solve(input: &str, loop_count: u32, divide_by: u64) -> u64 {
  let mut monkeys = input.split("\n\n").map(Monkey::from_str).collect_vec();
  let mod_to_rule_them_all: u64 = monkeys.iter().map(|m| m.test).product();

  for _ in 0..loop_count {
    for i in 0..monkeys.len() {
      while let Some(item) = monkeys[i].items.pop_front() {
        let item = (monkeys[i].apply_op(item) % mod_to_rule_them_all) / divide_by;

        let throw_to = monkeys[i].throw_to[usize::from(item % monkeys[i].test == 0)] as usize;

        monkeys[throw_to].items.push_back(item);
        monkeys[i].items_handeled += 1;
      }
    }
  }
  monkeys
    .iter()
    .map(|m| m.items_handeled)
    .sorted()
    .rev()
    .take(2)
    .product()
}

pub fn part_one(input: &str) -> Option<u64> {
  Some(solve(input, 20, 3))
}

pub fn part_two(input: &str) -> Option<u64> {
  Some(solve(input, 10000, 1))
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 11);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 11);
    assert_eq!(part_one(&input), Some(10605));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 11);
    assert_eq!(part_two(&input), Some(2713310158));
  }
}
