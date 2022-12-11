use itertools::Itertools;

fn parse<C, F>(input: &str, init: (i32, i32, Vec<C>), f: F) -> Vec<C>
where
  F: FnMut((i32, i32, Vec<C>), &i32) -> (i32, i32, Vec<C>),
{
  input
    .lines()
    .map(|op| match *op.split(' ').collect_vec() {
      ["addx", v] => vec![0, v.parse::<i32>().unwrap()],
      _ => vec![0],
    })
    .concat()
    .iter()
    .fold(init, f)
    .2
}

pub fn part_one(input: &str) -> Option<i32> {
  Some(
    parse(input, (1, 1, vec![]), |(x, pc, mut vals), op| {
      if pc % 40 == 20 {
        vals.push(x * pc);
      }
      (x + op, pc + 1, vals)
    })
    .iter()
    .sum(),
  )
}

fn do_op(x: &mut i32, pc: &mut i32, val: i32, output: &mut [Vec<char>], width: usize) {
  let distance = i32::abs(((*pc - 1) % 40) - *x);
  if distance < 2 {
    *output
      .get_mut((*pc as usize - 1) / width)
      .unwrap()
      .get_mut((*pc as usize - 1) % width)
      .unwrap() = '█';
  }
  *x += val;
  *pc += 1;
}

pub fn part_two(input: &str) -> Option<String> {
  let width = 40;
  let height = 6;
  let mut output = vec![vec![' '; width]; height];
  let mut x = 1;
  let mut pc = 1;

  for line in input.lines() {
    let val = match *line.split(' ').collect_vec() {
      ["addx", v] => v.parse::<i32>().unwrap(),
      _ => 0,
    };
    do_op(&mut x, &mut pc, 0, &mut output, width);
    if val != 0 {
      do_op(&mut x, &mut pc, val, &mut output, width);
    }
  }

  Some(
    output
      .iter()
      .map(|l| l.iter().collect::<String>())
      .join("\n"),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 10);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 10);
    assert_eq!(part_one(&input), Some(13140));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 10);
    assert_eq!(part_two(&input), Some("██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \n████    ████    ████    ████    ████    \n█████     █████     █████     █████     \n██████      ██████      ██████      ████\n███████       ███████       ███████     ".to_string()));
  }
}
