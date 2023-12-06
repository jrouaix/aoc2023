advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
  let mut lines = input.lines();
  let (Some(times), Some(distances)) = (lines.next(), lines.next()) else { return None };

  let times = times.split_whitespace().filter_map(|s| s.parse::<u64>().ok());
  let distances = distances.split_whitespace().filter_map(|s| s.parse::<u64>().ok());

  let races = std::iter::zip(times, distances).map(|(time, dist)| Race { time, dist }).collect();

  compute(races)
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut lines = input.lines();
  let (Some(times), Some(distances)) = (lines.next(), lines.next()) else { return None };

  let time = times.replace("Time:", "").replace(" ", "").parse().unwrap();
  let dist = distances.replace("Distance:", "").replace(" ", "").parse().unwrap();

  let race = Race { time, dist };

  compute(vec![race])
}

fn compute(races: Vec<Race>) -> Option<u64> {
  Some(
    races
      .into_iter()
      .map(|r| (1..r.time).map(|hold_time| r.end_dist(hold_time)).filter(|end_dist| *end_dist > r.dist).count() as u64)
      .fold(1, |acc, x| acc * x),
  )
}

#[derive(Debug)]
struct Race {
  time: u64,
  dist: u64,
}

impl Race {
  fn end_dist(&self, hold_time: u64) -> u64 {
    (self.time - hold_time).saturating_mul(hold_time)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r##"Time:      7  15   30
Distance:  9  40  200"##;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(138915));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(288));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(27340847));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(71503));
  }
}
