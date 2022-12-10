use itertools::Itertools;

fn solve<C: PartialEq + std::cmp::Ord, F>(input: &str, init: (i32, i32, Vec<C>), f: F) -> Vec<C>
where
  F: FnMut((i32, i32, Vec<C>), &i32) -> (i32, i32, Vec<C>),
{
  input
    .lines()
    .map(|op| match *op.split(' ').collect_vec() {
      ["noop"] => vec![0],
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
    solve(input, (1, 1, vec![]), |(x, pc, mut vals), op| {
      if pc % 40 == 20 {
        vals.push(x * pc);
      }
      (x + op, pc + 1, vals)
    })
    .iter()
    .sum(),
  )
}

pub fn part_two(input: &str) -> Option<String> {
  Some(
    solve(input, (1, 1, vec![]), |(x, pc, mut vals), op| {
      let y = i32::abs(((pc - 1) % 40) - x);
      vals.push(if y <= 1 { '#' } else { '.' });

      (x + op, pc + 1, vals)
    })
    .chunks(40)
    .into_iter()
    .intersperse(&vec!['\n'].to_vec())
    .map(|l| l.to_owned())
    .concat()
    .iter()
    .collect::<String>(),
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
    assert_eq!(part_two(&input), Some("##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....".to_string()));
  }
}
