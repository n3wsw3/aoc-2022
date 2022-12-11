use itertools::Itertools;
use std::collections::VecDeque;

type Size = u64;

enum Op {
  Add(Size),
  Multiply(Size),
  Square,
}

struct Monkey {
  items: VecDeque<Size>,
  op: Op,
  test: Size,
  throw_to: Vec<u32>,
  items_handeled: Size,
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
            .map(|v| v.parse::<Size>().unwrap())
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
}

fn solve<F>(input: &str, loop_count: u32, f: F) -> Size
where
  F: Fn(&mut Size, Size),
{
  let mut mod_to_rule_them_all: Size = 1;
  let mut monkeys = input.split("\n\n").map(Monkey::from_str).collect_vec();
  for monke in &monkeys {
    mod_to_rule_them_all *= monke.test;
  }
  for _ in 0..loop_count {
    for i in 0..monkeys.len() {
      let (first, rest) = monkeys.split_at_mut(i);
      let (monke, rest) = rest.split_first_mut().unwrap();
      while !monke.items.is_empty() {
        let item = monke.items.pop_front().unwrap();
        let mut item = match monke.op {
          Op::Multiply(v) => item * v,
          Op::Add(v) => item + v,
          Op::Square => item * item,
        };

        f(&mut item, mod_to_rule_them_all);

        let test = usize::from(item % monke.test == 0);
        let throw_to = *monke.throw_to.get(test).unwrap() as usize;

        if throw_to < i {
          first.get_mut(throw_to).unwrap().items.push_back(item);
        } else {
          rest
            .get_mut(throw_to - i - 1)
            .unwrap()
            .items
            .push_back(item);
        }

        monke.items_handeled += 1;
      }
    }
  }
  let x = monkeys
    .iter()
    .map(|m| m.items_handeled)
    .sorted()
    .rev()
    .take(2)
    .collect_vec();
  x.first().unwrap() * x.get(1).unwrap()
}

pub fn part_one(input: &str) -> Option<Size> {
  Some(solve(input, 20, |item, mod_num| {
    *item %= mod_num;
    *item /= 3;
  }))
}

pub fn part_two(input: &str) -> Option<Size> {
  Some(solve(input, 10000, |item, mod_num| {
    *item %= mod_num;
  }))
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
