use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq)]
enum NodeType {
  Directory,
  File,
}

struct Node {
  children: HashMap<String, Node>,
  node_type: NodeType,
  size: u32,
}

impl Node {
  fn new(node_type: NodeType, size: u32) -> Node {
    Node {
      children: HashMap::new(),
      node_type,
      size,
    }
  }

  fn size(&self) -> u32 {
    self.children.values().map(|node| node.size()).sum::<u32>() + self.size
  }

  fn get_dirs(&self) -> Vec<&Node> {
    self
      .children
      .values()
      .filter(|node| node.node_type == NodeType::Directory)
      .map(|node| {
        let mut dirs = node.get_dirs();
        dirs.push(node);
        dirs
      })
      .concat()
  }
}

fn traverse<'a>(root: &'a mut Node, path: &Vec<&str>) -> &'a mut Node {
  let mut x: &'a mut Node = root;
  for p in path {
    x = x.children.get_mut(p.to_owned()).unwrap();
  }

  x
}

fn parse(input: &str) -> Node {
  let mut root: Node = Node::new(NodeType::Directory, 0);
  let mut path: Vec<&str> = Vec::new();

  for (start, command) in input.lines().map(|line| line.split_once(' ').unwrap()) {
    let node = traverse(&mut root, &path);

    match start {
      "$" => {
        if command != "ls" {
          let (_, place) = command.split_once(' ').unwrap();

          match place {
            "/" => {
              path = Vec::new();
            }
            ".." => {
              path.pop();
            }
            _ => {
              if !node.children.contains_key(place) {
                node
                  .children
                  .insert(place.to_string(), Node::new(NodeType::Directory, 0));
              }
              path.push(place);
            }
          }
        }
      }
      "dir" => {
        node
          .children
          .insert(command.to_string(), Node::new(NodeType::Directory, 0));
      }
      _ => {
        node.children.insert(
          command.to_string(),
          Node::new(NodeType::File, start.parse::<u32>().unwrap()),
        );
      }
    }
  }
  root
}

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    parse(input)
      .get_dirs()
      .into_iter()
      .map(|node| node.size())
      .filter(|node_size| node_size <= &100000)
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  let root = parse(input);

  let mut dirs = root.get_dirs();
  dirs.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());

  let total: u32 = 70000000;
  let needed: u32 = 30000000;
  let used: u32 = root.size();

  let need = needed - (total - used);

  let delete_node = dirs.iter().find(|node| node.size() >= need).unwrap();

  Some(delete_node.size())
}

fn main() {
  let input = &advent_of_code::read_file("inputs", 7);
  advent_of_code::solve!(1, part_one, input);
  advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = advent_of_code::read_file("examples", 7);
    assert_eq!(part_one(&input), Some(95437));
  }

  #[test]
  fn test_part_two() {
    let input = advent_of_code::read_file("examples", 7);
    assert_eq!(part_two(&input), Some(24933642));
  }
}
