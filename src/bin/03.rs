use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one<'a>(input: &'a str) -> Option<u32> {
  let digit_regex = regex::Regex::new(r"\d+").unwrap();
  type Bag = HashSet<u32>;
  let mut bag = Bag::new();

  fn char_is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.' || c == '\n')
  }

  fn slice_is_adjacent(slice: &str) -> bool {
    slice.chars().any(char_is_symbol)
  }

  fn acc_fn<'a>(digit_regex: &regex::Regex, bag: &mut Bag, prev: &'a str, line: &'a str, next: &'a str, mut sum: u32) -> (u32, &'a str, &'a str) {
    for r#match in digit_regex.find_iter(line) {
      let left = r#match.start().saturating_sub(1);
      let right = r#match.end().saturating_add(1);

      // let slices = || {
      //   format!("\n{}\n{}\n{}\n-------", prev.get(left..right).unwrap_or_default(), line.get(left..right).unwrap_or_default(), next.get(left..right).unwrap_or_default())
      // };

      if slice_is_adjacent(prev.get(left..right).unwrap_or_default())
        || slice_is_adjacent(line.get(left..right).unwrap_or_default())
        || slice_is_adjacent(next.get(left..right).unwrap_or_default())
      {
        let number = r#match.as_str().parse::<u32>().unwrap_or_default();
        bag.insert(number);
        sum += number;
      }
      // else {
      //   println!("{}", slices());
      // }
    }

    (sum, line, next)
  }

  let (sum, last_prev, last_line) = input
    .lines()
    .fold((0_u32, "", "") as (u32, &'a str, &'a str), |(sum, prev, line), next| acc_fn(&digit_regex, &mut bag, prev, line, next, sum));

  let (sum, _, _) = acc_fn(&digit_regex, &mut bag, last_prev, last_line, "", sum);

  Some(bag.into_iter().sum())
  // Some(sum)
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

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(4361));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, None);
  }
}
