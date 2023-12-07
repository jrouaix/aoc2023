advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Play {
  hand: Hand,
  bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
  _A,
  _K,
  _Q,
  _J,
  _T,
  _9,
  _8,
  _7,
  _6,
  _5,
  _4,
  _3,
  _2,
}

impl TryFrom<char> for Card {
  type Error = &'static str;
  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      'A' => Ok(Self::_A),
      'K' => Ok(Self::_K),
      'Q' => Ok(Self::_Q),
      'J' => Ok(Self::_J),
      'T' => Ok(Self::_T),
      '9' => Ok(Self::_9),
      '8' => Ok(Self::_8),
      '7' => Ok(Self::_7),
      '6' => Ok(Self::_6),
      '5' => Ok(Self::_5),
      '4' => Ok(Self::_4),
      '3' => Ok(Self::_3),
      '2' => Ok(Self::_2),
      _ => Err("Invalid card"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
  FiveOfAKind,
  FourOfAKind,
  FullHouse,
  ThreeOfAKind,
  TwoPairs,
  OnePair,
  HighCard,
}

use std::cmp::Ordering;

type InnerHand = [Card; 5];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Hand {
  strength: Strength,
  inner: InnerHand,
}

impl Hand {
  fn new(inner: InnerHand) -> Self {
    let mut counts = [0_usize; 13];
    for card in inner.iter() {
      counts[*card as usize] += 1;
    }
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let strength = match counts {
      [5, ..] => Strength::FiveOfAKind,
      [4, ..] => Strength::FourOfAKind,
      [3, 2, ..] => Strength::FullHouse,
      [3, ..] => Strength::ThreeOfAKind,
      [2, 2, ..] => Strength::TwoPairs,
      [2, ..] => Strength::OnePair,
      _ => Strength::HighCard,
    };

    Self { strength, inner }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.strength != other.strength {
      return self.strength.partial_cmp(&other.strength);
    }
    std::iter::zip(self.inner.iter(), other.inner.iter())
      .find_map(|(a, b)| a.partial_cmp(b))
      .or_else(|| Some(Ordering::Equal))
  }
}

pub fn part_one(input: &str) -> Option<u64> {
  let mut plays: Vec<Play> = input
    .lines()
    .filter_map(|l| {
      let mut parts = l.split_whitespace();
      let (Some(hand), Some(bid)) = (parts.next(), parts.next()) else {
        return None;
      };

      let mut cards = hand.chars();
      let (Some(c1), Some(c2), Some(c3), Some(c4), Some(c5)) = (cards.next(), cards.next(), cards.next(), cards.next(), cards.next()) else {
        return None;
      };

      let inner = [Card::try_from(c1).unwrap(), Card::try_from(c2).unwrap(), Card::try_from(c3).unwrap(), Card::try_from(c4).unwrap(), Card::try_from(c5).unwrap()];
      let hand = Hand::new(inner);

      let bid = bid.parse().unwrap();

      let play = Play { hand, bid };
      // dbg!(l, &play);
      Some(play)
    })
    .collect();

  plays.sort_by(|a, b| b.hand.cmp(&a.hand));

  Some(plays.iter().enumerate().map(|(i, play)| (i as u64 + 1) * play.bid).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = r##"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"##;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, Some(249390788));
  }

  #[test]
  fn test_part_one_example() {
    let result = part_one(EXAMPLE);
    assert_eq!(result, Some(6440));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
    assert_eq!(result, None);
  }
}
