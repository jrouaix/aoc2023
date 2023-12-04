advent_of_code::solution!(4);

fn get_values<'a>(winnings_and_have: Option<&'a str>) -> impl Iterator<Item = u32> + 'a {
  winnings_and_have.unwrap_or_default().split_whitespace().filter_map(|s| s.parse::<u32>().ok())
}

pub fn part_one(input: &str) -> Option<u32> {
  input.lines().fold(None, |sum, line| {
    let mut card_and_numbers = line.split(":").into_iter();
    let _ignored_card = card_and_numbers.next()?;
    let mut winnings_and_have = card_and_numbers.next()?.split("|");

    let winnings = get_values(winnings_and_have.next()).collect::<Vec<_>>();
    let have_count = get_values(winnings_and_have.next()).filter(|n| winnings.contains(n)).count();

    if have_count == 0 {
      sum
    } else {
      let line_worth = 2_u32.pow(have_count as u32 - 1);
      let new_sum = sum.unwrap_or_default() + line_worth;
      Some(new_sum)
    }
  })
}

pub fn part_two(input: &str) -> Option<u32> {
  Some(
    input
      .lines()
      .fold((0, vec![]), |acc: (_, Vec<u32>), line: &str| {
        let (sum, mut copies) = acc;
        let mut card_and_numbers = line.split(":").into_iter();
        let _ignored_card = card_and_numbers.next();

        let Some(winnings_and_have) = card_and_numbers.next() else {
          return (sum, copies);
        };
        let mut winnings_and_have = winnings_and_have.split("|");

        let winnings = get_values(winnings_and_have.next()).collect::<Vec<_>>();

        let copies_of_line = if copies.is_empty() { 0 } else { copies.remove(0) };
        let to_add = copies_of_line + 1;

        let sum = sum + to_add;

        let have_count = get_values(winnings_and_have.next()).filter(|n| winnings.contains(n)).count();

        for (_, i) in (0..have_count).enumerate() {
          if i < copies.len() {
            copies[i] += to_add as u32;
          } else {
            copies.push(to_add as u32)
          }
        }

        (sum, copies)
      })
      .0,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(21138));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(&EXAMPLE);
    assert_eq!(result, Some(13));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(7185540));
  }
  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(30));
  }
}
