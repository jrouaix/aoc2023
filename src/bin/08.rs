use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug)]
struct Document {
  instructions: Vec<Instr>,
  nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumString)]
enum Instr {
  L,
  R,
}

#[derive(Debug)]
struct Node {
  _id: NodeId,
  left: NodeId,
  right: NodeId,
}

type NodeId = String;

pub fn part_one(input: &str) -> Option<u64> {
  let document = parse_input(input);
  Some(get_steps(&document, "AAA".into(), |id| id == "ZZZ"))
}

pub fn part_two(input: &str) -> Option<u64> {
  let document = parse_input(input);

  let solutions = document
    .nodes
    .iter()
    .filter(|(id, _)| id.ends_with("A"))
    .map(|(id, _)| get_steps(&document, id.clone(), |id| id.ends_with("Z")))
    .collect::<Vec<_>>();

  dbg!(&solutions);

  Some(solutions.into_iter().fold(1, |acc, s| lcm(acc, s)))
}

fn get_steps(document: &Document, start_node: NodeId, is_arrived: impl Fn(&str) -> bool) -> u64 {
  let mut i = 0;
  let mut current_node: NodeId = start_node;
  let mut count = 0;
  loop {
    let current = document.nodes.get(&current_node).unwrap_or_else(|| panic!("Node {current_node} not found"));
    let instr = document.instructions.get(i).unwrap_or_else(|| panic!("Instruction {i} not found"));

    match instr {
      Instr::L => current_node = current.left.clone(),
      Instr::R => current_node = current.right.clone(),
    }

    i += 1;
    if i >= document.instructions.len() {
      i = 0;
    }
    count += 1;
    if is_arrived(&current_node) {
      return count;
    }
  }
}

fn parse_input(input: &str) -> Document {
  let mut lines = input.lines();
  let instructions = lines
    .next()
    .expect("Should be a least one line")
    .chars()
    .map(|c| c.to_string().parse::<Instr>().unwrap())
    .collect();

  let nodes = lines
    .into_iter()
    .filter_map(|line| {
      if !line.contains("=") {
        return None;
      }

      let line = line.replace("=", "").replace("(", "").replace(")", "").replace(",", "");
      let mut parts = line.split_whitespace();
      let (id, left, right) = (parts.next().expect("missing node id"), parts.next().expect("missing left node"), parts.next().expect("missing right node"));

      Some((id.to_string(), Node { _id: id.to_string(), left: left.to_string(), right: right.to_string() }))
    })
    .collect();

  Document { instructions, nodes }
}

/// GIVEN BY CHAT GPT

fn lcm(a: u64, b: u64) -> u64 {
  (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

/// GIVEN BY CHAT GPT

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(22411));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(2));
  }

  const EXAMPLE2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(11188774513823));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE2);
    assert_eq!(result, Some(6));
  }
}
