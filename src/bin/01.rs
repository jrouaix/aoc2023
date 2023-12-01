use nom::{
  branch::alt,
  bytes::complete::{tag_no_case, take_while_m_n},
  character::complete::anychar,
  combinator::{iterator, map, map_res, value},
  IResult,
};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
  Some(
    input
      .lines()
      .map(|l| {
        l.chars().filter(|c| c.is_digit(10)).fold((None, None), |(first, last), c| {
          let i = c.to_string().parse::<u32>().unwrap();
          match (first, last) {
            (None, _) => (Some(i), Some(i)),
            (Some(f), _) => (Some(f), Some(i)),
          }
        })
      })
      .filter_map(|(first, last)| first.and_then(|first| last.map(|last| first * 10 + last)))
      .sum(),
  )
}

pub fn part_two(input: &str) -> Option<u32> {
  fn digit(input: &str) -> IResult<&str, Option<u32>> {
    map_res(take_while_m_n(1, 1, |c: char| c.is_digit(10)), |s| str::parse(s).map(Some))(input)
  }
  fn in_letters(input: &str) -> IResult<&str, Option<u32>> {
    map(
      alt((
        value(1, tag_no_case("one")),
        value(2, tag_no_case("two")),
        value(3, tag_no_case("three")),
        value(4, tag_no_case("four")),
        value(5, tag_no_case("five")),
        value(6, tag_no_case("six")),
        value(7, tag_no_case("seven")),
        value(8, tag_no_case("eight")),
        value(9, tag_no_case("nine")),
      )),
      |v| Some(v),
    )(input)
  }
  fn in_letters_reversed(input: &str) -> IResult<&str, Option<u32>> {
    map(
      alt((
        value(1, tag_no_case("eno")),
        value(2, tag_no_case("owt")),
        value(3, tag_no_case("eerht")),
        value(4, tag_no_case("ruof")),
        value(5, tag_no_case("evif")),
        value(6, tag_no_case("xis")),
        value(7, tag_no_case("neves")),
        value(8, tag_no_case("thgie")),
        value(9, tag_no_case("enin")),
      )),
      |v| Some(v),
    )(input)
  }

  fn first(input: &str) -> Option<u32> {
    iterator(input, alt((digit, in_letters, value(None, anychar)))).into_iter().flatten().next()
  }
  fn last(input: &str) -> Option<u32> {
    let input = input.chars().rev().collect::<String>();
    let input = input.as_str();
    let result = iterator(input, alt((digit, in_letters_reversed, value(None, anychar))))
      .into_iter()
      .flatten()
      .next();
    result
  }

  Some(
    input
      .lines()
      .map(|l| (first(l), last(l)))
      .filter_map(|(first, last)| first.and_then(|first| last.map(|last| first * 10 + last)))
      .sum(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(54597));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(54504));
  }

  #[test]
  fn test_part_two2() {
    let result = part_two(
      r"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    ",
    );
    assert_eq!(result, Some(281));
  }
}
