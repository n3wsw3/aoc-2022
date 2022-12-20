use itertools::Itertools;

struct Blueprint {
  robots: [[u16; 4]; 4],
}

fn parse(input: &str) -> Vec<Blueprint> {
  input
    .lines()
    .map(|line| {
      let x = line.split(' ').collect_vec();
      let ore_robot_cost = [x[6].parse().unwrap(), 0, 0, 0];
      let clay_robot_cost = [x[12].parse().unwrap(), 0, 0, 0];
      let obsidian_robot_cost = [x[18].parse().unwrap(), x[21].parse().unwrap(), 0, 0];
      let geode_robot_cost = [x[27].parse().unwrap(), 0, x[30].parse().unwrap(), 0];

      Blueprint {
        robots: [
          ore_robot_cost,
          clay_robot_cost,
          obsidian_robot_cost,
          geode_robot_cost,
        ],
      }
    })
    .collect()
}

type OreAmount = u16;

#[derive(Debug)]
struct State {
  ores: [OreAmount; 4],
  robots: [u16; 4],
  time: u16,
}

fn simulate(
  blueprint: &Blueprint,
  state: State,
  max_time: u16,
  max_robots: &[u16; 4],
  max_geodes: &mut OreAmount,
) {
  let mut has_recursed = false;
  for i in 0..4 {
    if state.robots[i] == max_robots[i] {
      continue;
    }
    let recipe = &blueprint.robots[i];
    // Find the limiting ore for the recipe.
    let wait_time = (0..3)
      .filter_map(|ore_type| {
        if recipe[ore_type] == 0 {
          None
        } else if recipe[ore_type] <= state.ores[ore_type] {
          Some(0)
        } else if state.robots[ore_type] == 0 {
          // No robot yet, we can't build it (it takes more than max_time to build it).
          Some(max_time as u16 + 1)
        } else {
          Some(
            (recipe[ore_type] - state.ores[ore_type] + state.robots[ore_type] - 1)
              / state.robots[ore_type],
          )
        }
      })
      .max()
      .unwrap();
    let time_finished = state.time + wait_time + 1;
    if time_finished >= max_time {
      continue;
    }
    let mut new_ores = [0; 4];
    let mut new_robots = [0; 4];
    for o in 0..4 {
      new_ores[o] = state.ores[o] + state.robots[o] * (wait_time + 1) - recipe[o];
      new_robots[o] = state.robots[o] + u16::from(o == i);
    }
    let remaining_time = max_time - time_finished;
    // If we were to build only geode robots every turn, could we beat the current max?
    if ((remaining_time - 1) * remaining_time) / 2 + new_ores[3] + remaining_time * new_robots[3]
      < *max_geodes
    {
      continue;
    }
    has_recursed = true;
    simulate(
      blueprint,
      State {
        ores: new_ores,
        robots: new_robots,
        time: time_finished,
      },
      max_time,
      max_robots,
      max_geodes,
    );
  }
  if !has_recursed {
    // We couldn't make new robots, so this is the best this branch can do.
    *max_geodes = std::cmp::max(
      *max_geodes,
      state.ores[3] + state.robots[3] * (max_time - state.time) as u16,
    );
  }
}

fn run_blueprint(blueprint: &Blueprint, max_time: u16) -> OreAmount {
  let mut max_robots = [u16::max_value(); 4];
  for i in 0..3 {
    max_robots[i] = blueprint.robots.iter().map(|r| r[i]).max().unwrap();
  }
  let mut max_geodes = 0;
  simulate(
    blueprint,
    State {
      ores: [0; 4],
      robots: [1, 0, 0, 0],
      time: 0,
    },
    max_time,
    &max_robots,
    &mut max_geodes,
  );
  max_geodes
}

pub fn part_one(input: &str) -> Option<usize> {
  let blueprints = parse(input);

  Some(
    blueprints
      .iter()
      .enumerate()
      .map(|(i, b)| run_blueprint(b, 24) as usize * (i + 1))
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<usize> {
  let blueprints = parse(input);

  Some(
    blueprints
      .iter()
      .take(3)
      .map(|b| run_blueprint(b, 32) as usize)
      .product(),
  )
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 19);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 19);
    assert_eq!(part_one(&input), None);
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 19);
    assert_eq!(part_two(&input), None);
  }
}
