use nom::{
  branch::alt,
  bytes::complete::{tag_no_case, take_while_m_n},
  character::complete::anychar,
  combinator::{map, map_res, value},
  multi::many0,
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
  fn digit(input: &str) -> IResult<&str, u32> {
    map_res(take_while_m_n(1, 1, |c: char| c.is_digit(10)), str::parse)(input)
  }
  fn in_letters(input: &str) -> IResult<&str, u32> {
    alt((
      // value(0, tag_no_case("zero")),
      value(1, tag_no_case("one")),
      value(2, tag_no_case("two")),
      value(3, tag_no_case("three")),
      value(4, tag_no_case("four")),
      value(5, tag_no_case("five")),
      value(6, tag_no_case("six")),
      value(7, tag_no_case("seven")),
      value(8, tag_no_case("eight")),
      value(9, tag_no_case("nine")),
    ))(input)
  }
  fn junk(input: &str) -> IResult<&str, ()> {
    value((), anychar)(input)
  }
  fn line(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    many0(alt((map(digit, |i| Some(i)), map(in_letters, |i| Some(i)), value(None, junk))))(input)
  }

  Some(
    input
      .lines()
      .map(|l| {
        dbg!(l);
        dbg!(line(l).unwrap().1.into_iter().flatten()).fold((None, None), |(first, last), i| match (first, last) {
          (None, _) => (Some(i), Some(i)),
          (Some(f), _) => (Some(f), Some(i)),
        })
      })
      .filter_map(|(first, last)| first.and_then(|first| last.map(|last| dbg!(first) * 10 + dbg!(last))))
      .sum(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(37492));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(54506));
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
