use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Meth<'a> {
  Val(f64),
  Add(&'a str, &'a str),
  Sub(&'a str, &'a str),
  Mul(&'a str, &'a str),
  Div(&'a str, &'a str),
}

fn has_human(name: &str, map: &HashMap<&str, Meth>) -> bool {
  if name == "humn" {
    return true;
  }

  match map.get(name) {
    Some(m) => match m {
      Meth::Add(n1, n2) => has_human(n1, map) || has_human(n2, map),
      Meth::Sub(n1, n2) => has_human(n1, map) || has_human(n2, map),
      Meth::Mul(n1, n2) => has_human(n1, map) || has_human(n2, map),
      Meth::Div(n1, n2) => has_human(n1, map) || has_human(n2, map),
      Meth::Val(_) => false,
    },
    None => false,
  }
}

fn calculate(name: &str, map: &HashMap<&str, Meth>) -> Result<f64, String> {
  match map.get(name) {
    Some(x) => match x {
      Meth::Val(x) => Ok(*x),
      Meth::Add(n1, n2) => Ok(calculate(n1, map)? + calculate(n2, map)?),
      Meth::Sub(n1, n2) => Ok(calculate(n1, map)? - calculate(n2, map)?),
      Meth::Mul(n1, n2) => Ok(calculate(n1, map)? * calculate(n2, map)?),
      Meth::Div(n1, n2) => Ok(calculate(n1, map)? / calculate(n2, map)?),
    },
    None => Err("Cannot find name".to_string()),
  }
}

fn parse(input: &str) -> HashMap<&str, Meth> {
  let mut map: HashMap<&str, Meth> = HashMap::new();

  for line in input.lines() {
    match line.split(' ').collect_vec()[..] {
      [name, val1, op, val2] => {
        let name = &name[0..4];
        match op {
          "+" => {
            map.insert(name, Meth::Add(val1, val2));
          }
          "-" => {
            map.insert(name, Meth::Sub(val1, val2));
          }
          "*" => {
            map.insert(name, Meth::Mul(val1, val2));
          }
          "/" => {
            map.insert(name, Meth::Div(val1, val2));
          }
          _ => {
            unreachable!("OP did fucky wucky");
          }
        }
      }
      [name, val] => {
        let name = &name[0..4];
        map.insert(name, Meth::Val(val.parse().unwrap()));
      }
      _ => {
        unreachable!("Parsing line did fucky wucky")
      }
    }
  }
  map
}

pub fn part_one(input: &str) -> Option<u64> {
  let map = parse(input);

  Some(calculate("root", &map).unwrap() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut map = parse(input);

  let (humn, other) = match map["root"] {
    Meth::Add(n1, n2) => {
      if has_human(n1, &map) {
        (n1, n2)
      } else {
        (n2, n1)
      }
    }
    _ => return None,
  };

  let x1 = 1f64;
  let x2 = 100000000000000000f64;

  map.insert("humn", Meth::Val(x1));
  let first = calculate(humn, &map).unwrap();

  map.insert("humn", Meth::Val(x2));
  let second = calculate(humn, &map).unwrap();

  Some(((calculate(other, &map).unwrap() - first) * (x2 - x1) / (second - first) + x1) as u64)
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 21);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 21);
    assert_eq!(part_one(&input), Some(152));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 21);
    assert_eq!(part_two(&input), Some(301));
  }
}
