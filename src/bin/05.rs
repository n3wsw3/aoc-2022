use itertools::Itertools;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
  assert!(!v.is_empty());
  let len = v[0].len();
  let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
  (0..len)
    .map(|_| {
      iters
        .iter_mut()
        .map(|n| n.next().unwrap())
        .collect::<Vec<T>>()
    })
    .collect()
}

fn parse_stack_line(line: &str) -> Vec<char> {
  line
    .chars()
    .chunks(4)
    .into_iter()
    .map(|mut chunk| chunk.nth(1).unwrap())
    .collect_vec()
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
  transpose(
    input
      .lines()
      .filter(|line| line.contains("["))
      .map(parse_stack_line)
      .rev()
      .collect_vec(),
  )
  .into_iter()
  .map(|line| line.into_iter().filter(|c| *c != ' ').collect_vec())
  .collect_vec()
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
  input
    .lines()
    .map(|line| {
      line
        .split(' ')
        .filter_map(|i| i.parse().ok())
        .collect_tuple::<(usize, usize, usize)>()
        .unwrap()
    })
    .collect_vec()
}

fn last_n(stacks: &mut Vec<char>, at: usize) -> Vec<char> {
  let mut new = Vec::new();

  for _ in 0..at {
    new.push(stacks.pop().unwrap());
  }

  new.reverse();
  new
}

pub fn part_one(input: &str) -> Option<String> {
  let (stacks_input, instructions_input) = input.split("\n\n").collect_tuple().unwrap();
  let mut stacks = parse_stacks(stacks_input);
  parse_instructions(instructions_input).into_iter().for_each(|(amount, from, to)| {
    for _ in 0..amount {
      let value = stacks.get_mut(from - 1).unwrap().pop().unwrap();
      stacks.get_mut(to - 1).unwrap().push(value)
    }
  });

  Some(stacks.into_iter().filter_map(|stack| stack.last().cloned()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
  let (stacks_input, instructions_input) = input.split("\n\n").collect_tuple().unwrap();
  let mut stacks = parse_stacks(stacks_input);
  parse_instructions(instructions_input).into_iter().for_each(|(amount, from, to)| {
    let mut value = last_n(stacks.get_mut(from - 1).unwrap(), amount);
    stacks.get_mut(to - 1).unwrap().append(&mut value)
  });
  
  Some(stacks.into_iter().filter_map(|stack| stack.last().cloned()).collect())
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 5);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 5);
    assert_eq!(part_one(&input), Some("CMZ".to_owned()));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 5);
    assert_eq!(part_two(&input), Some("MCD".to_owned()));
  }
}
