use nom::{
  branch::alt,
  bytes::complete::tag_no_case,
  character::complete::{char, digit1, space0, space1},
  combinator::{map, map_res, value},
  multi::separated_list0,
  sequence::tuple,
  IResult,
};

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
enum Color {
  Green,
  Blue,
  Red,
}

#[derive(Debug, Clone)]
pub struct Game {
  id: u32,
  hands: Vec<Hand>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Hand {
  red: u32,
  blue: u32,
  green: u32,
}

fn number(input: &str) -> IResult<&str, u32> {
  map_res(digit1::<&str, _>, |digits| digits.parse::<u32>())(input)
}
fn game_id(input: &str) -> IResult<&str, u32> {
  map(tuple((tag_no_case("Game "), number)), |(_, id)| id)(input)
}
fn cube_color(input: &str) -> IResult<&str, Color> {
  alt((value(Color::Green, tag_no_case("green")), value(Color::Blue, tag_no_case("blue")), value(Color::Red, tag_no_case("red"))))(input)
}
fn cube_count(input: &str) -> IResult<&str, (u32, Color)> {
  map(tuple((space0, number, space1, cube_color)), |(_, count, _, color)| (count, color))(input)
}
fn hand(input: &str) -> IResult<&str, Hand> {
  map(separated_list0(char(','), cube_count), |v| {
    let mut hand = Hand::default();
    for (count, color) in v {
      match color {
        Color::Green => hand.green += count,
        Color::Blue => hand.blue += count,
        Color::Red => hand.red += count,
      }
    }
    hand
  })(input)
}
fn game(input: &str) -> IResult<&str, Game> {
  map(tuple((game_id, char(':'), separated_list0(char(';'), hand))), |(id, _, hands)| Game { id, hands })(input)
}

pub fn part_one(input: &str) -> Option<u32> {
  let (r_limit, g_limit, b_limit) = (12_u32, 13_u32, 14_u32);
  Some(
    input
      .lines()
      .filter_map(|l| game(l).map(|(_, g)| g).ok())
      .filter_map(|g| {
        for h in g.hands {
          if h.red > r_limit || h.green > g_limit || h.blue > b_limit {
            return None;
          }
        }
        Some(g.id)
      })
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    input
      .lines()
      .filter_map(|l| game(l).map(|(_, g)| g).ok())
      .map(|g| {
        let mut min_hand = Hand::default();
        for h in g.hands {
          if h.red > min_hand.red {
            min_hand.red = h.red;
          }
          if h.green > min_hand.green {
            min_hand.green = h.green;
          }
          if h.blue > min_hand.blue {
            min_hand.blue = h.blue;
          }
        }
        min_hand.red * min_hand.green * min_hand.blue
      })
      .sum(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(2447));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(8));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(56322));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(2286));
  }
}
