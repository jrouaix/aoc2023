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

pub fn part_two<'a>(input: &'a str) -> Option<u32> {
  // hash of gear positions and adjacents part numbers count & ratio
  type Bag = std::collections::BTreeMap<(usize, usize), (u32, u32)>;

  fn char_is_gear(c: char) -> bool {
    c == '*'
  }

  fn acc_fn<'a>(mut bag: Bag, line_number: usize, prev: &'a str, line: &'a str, next: &'a str) -> (Bag, usize, &'a str, &'a str) {
    for r#match in DIGITS.find_iter(line) {
      let left = r#match.start().saturating_sub(1);
      let right = r#match.end().saturating_add(1);

      let number = r#match.as_str().parse::<u32>().expect("Could not parse number");

      for (pos, c) in prev[min(left, prev.len())..min(right, prev.len())].chars().enumerate() {
        if char_is_gear(c) {
          let entry = bag.entry((line_number - 1, pos + left)).or_insert((0, 1));
          entry.0 += 1;
          if entry.0 < 3 {
            entry.1 *= number;
          }
        }
      }
      for (pos, c) in line[min(left, line.len())..min(right, line.len())].chars().enumerate() {
        if char_is_gear(c) {
          let entry = bag.entry((line_number, pos + left)).or_insert((0, 1));
          entry.0 += 1;
          if entry.0 < 3 {
            entry.1 *= number;
          }
        }
      }
      for (pos, c) in next[min(left, next.len())..min(right, next.len())].chars().enumerate() {
        if char_is_gear(c) {
          let entry = bag.entry((line_number + 1, pos + left)).or_insert((0, 1));
          entry.0 += 1;
          if entry.0 < 3 {
            entry.1 *= number;
          }
        }
      }
    }

    (bag, line_number + 1, line, next)
  }

  let (bag, last_line_number, last_prev, last_line) = input
    .lines()
    .fold((Bag::new(), 1_usize, "", "") as (_, _, &'a str, &'a str), |(bag, line_number, prev, line), next| acc_fn(bag, line_number, prev, line, next));

  let (bag, ..) = acc_fn(bag, last_line_number, last_prev, last_line, "");

  let ratios_sum = bag.into_iter().filter(|(_, (count, _))| *count == 2).map(|(_, (_, ratio))| ratio).sum::<u32>();

  Some(ratios_sum)
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
    assert_eq!(result, Some(536202));
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
    assert_eq!(result, Some(78272573));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(467835));
  }
}
