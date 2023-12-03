use std::cmp::min;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(3);

static DIGITS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

pub fn part_one<'a>(input: &'a str) -> Option<u32> {
  fn char_is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.' || c == '\n')
  }

  fn slice_contains_symbol(slice: &str) -> bool {
    slice.chars().any(char_is_symbol)
  }

  fn acc_fn<'a>(prev: &'a str, line: &'a str, next: &'a str, mut sum: u32) -> (u32, &'a str, &'a str) {
    for r#match in DIGITS.find_iter(line) {
      let left = r#match.start().saturating_sub(1);
      let right = r#match.end().saturating_add(1);

      if slice_contains_symbol(&prev[min(left, prev.len())..min(right, prev.len())])
        || slice_contains_symbol(&line[min(left, line.len())..min(right, line.len())])
        || slice_contains_symbol(&next[min(left, next.len())..min(right, next.len())])
      {
        let number = r#match.as_str().parse::<u32>().unwrap_or_default();
        sum += number;
      }
    }

    (sum, line, next)
  }

  let (sum, last_prev, last_line) = input
    .lines()
    .fold((0_u32, "", "") as (u32, &'a str, &'a str), |(sum, prev, line), next| acc_fn(prev, line, next, sum));

  let (sum, _, _) = acc_fn(last_prev, last_line, "", sum);

  Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

  const EXAMPLE2: &str = r"
123
.$.
123
";

  const EXAMPLE3: &str = r"
..123..
123#123
..123..
";

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(332942));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(4361));
  }

  #[test]
  fn test_part_one_example2() {
    let result = part_one(EXAMPLE2);
    assert_eq!(result, Some(246));
  }

  #[test]
  fn test_part_one_example3() {
    let result = part_one(EXAMPLE3);
    assert_eq!(result, Some(246 * 2));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(467835));
  }
}
