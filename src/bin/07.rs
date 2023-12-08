use std::{cmp::Ordering, marker::PhantomData};

advent_of_code::solution!(7);

use Card::*;
use Strength::*;

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
      'A' => Ok(_A),
      'K' => Ok(_K),
      'Q' => Ok(_Q),
      'J' => Ok(_J),
      'T' => Ok(_T),
      '9' => Ok(_9),
      '8' => Ok(_8),
      '7' => Ok(_7),
      '6' => Ok(_6),
      '5' => Ok(_5),
      '4' => Ok(_4),
      '3' => Ok(_3),
      '2' => Ok(_2),
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

type Hand = [Card; 5];

trait Rule: Sized + std::fmt::Debug {
  // fn ord(a: &Play<Self>, b: &Play<Self>) -> Ordering;
  fn value(card: &Card) -> u64;
  fn strength(h: Hand) -> Strength;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Play<R: Rule> {
  _rule: PhantomData<R>,
  hand: Hand,
  strength: Strength,
  bid: u64,
}

impl<R: Rule> Play<R> {
  fn new(hand: Hand, bid: u64) -> Self {
    let strength = R::strength(hand);
    Self { hand, strength, bid, _rule: PhantomData }
  }
}

#[derive(Debug)]
struct P1;
impl Rule for P1 {
  fn value(card: &Card) -> u64 {
    *card as u64
  }

  fn strength(hand: Hand) -> Strength {
    let mut counts = [0_usize; 13];
    for card in hand.iter() {
      counts[*card as usize] += 1;
    }
    counts.sort_unstable_by(|a, b| b.cmp(a));

    match counts {
      [5, ..] => FiveOfAKind,
      [4, ..] => FourOfAKind,
      [3, 2, ..] => FullHouse,
      [3, ..] => ThreeOfAKind,
      [2, 2, ..] => TwoPairs,
      [2, ..] => OnePair,
      _ => HighCard,
    }
  }
}

fn get_inner_hand(hand: &str) -> Option<Hand> {
  let mut cards = hand.chars().filter_map(|c| Card::try_from(c).ok());
  let (Some(c1), Some(c2), Some(c3), Some(c4), Some(c5)) = (cards.next(), cards.next(), cards.next(), cards.next(), cards.next()) else {
    unimplemented!("Invalid hand: {hand}");
  };
  Some([c1, c2, c3, c4, c5])
}

pub fn part_one(input: &str) -> Option<u64> {
  compute::<P1>(input)
}

#[derive(Debug)]
struct P2;
impl Rule for P2 {
  fn value(card: &Card) -> u64 {
    match card {
      _A => 0,
      _K => 1,
      _Q => 2,
      _T => 3,
      _9 => 4,
      _8 => 5,
      _7 => 6,
      _6 => 7,
      _5 => 8,
      _4 => 9,
      _3 => 10,
      _2 => 11,
      _J => 12,
    }
  }

  fn strength(hand: Hand) -> Strength {
    let mut counts = [0_usize; 13];
    let mut j_count = 0_usize;
    for card in hand.iter() {
      if *card == _J {
        j_count += 1;
      } else {
        counts[*card as usize] += 1;
      }
    }

    counts.sort_unstable_by(|a, b| b.cmp(a));

    match (counts, j_count) {
      ([5, ..], _) => FiveOfAKind,

      ([4, ..], 1..) => FiveOfAKind,
      ([4, ..], 0..) => FourOfAKind,

      ([3, 2, ..], _) => FullHouse,

      ([3, ..], 0) => ThreeOfAKind,
      ([3, ..], 1) => FourOfAKind,
      ([3, ..], 2) => FiveOfAKind,

      ([2, 2, ..], 0) => TwoPairs,
      ([2, 2, ..], 1) => FullHouse,

      ([2, ..], 0) => OnePair,
      ([2, ..], 1) => ThreeOfAKind,
      ([2, ..], 2) => FourOfAKind,
      ([2, ..], 3) => FiveOfAKind,

      ([1, ..], 0) => HighCard,
      ([1, ..], 1) => OnePair,
      ([1, ..], 2) => ThreeOfAKind,
      ([1, ..], 3) => FourOfAKind,
      ([1, ..], 4) => FiveOfAKind,

      (_, 5) => FiveOfAKind,

      _ => panic!("I don't know what i'm doing !! : {hand:?}"),
    }
  }
}

pub fn part_two(input: &str) -> Option<u64> {
  compute::<P2>(input)
}

fn compute<R: Rule>(input: &str) -> Option<u64> {
  let mut plays: Vec<Play<R>> = input
    .lines()
    .filter_map(|l| {
      let mut parts = l.split_whitespace();
      let (Some(hand), Some(bid)) = (parts.next(), parts.next()) else {
        return None;
      };

      let hand = get_inner_hand(hand)?;
      let bid = bid.parse().unwrap();

      let play = Play::new(hand, bid);
      Some(play)
    })
    .collect();

  plays.sort_by(|a, b| {
    if a.strength != b.strength {
      b.strength.cmp(&a.strength)
    } else {
      std::iter::zip(a.hand.iter(), b.hand.iter())
        .map(|(a, b)| (R::value(a), R::value(b)))
        .map(|(a, b)| b.cmp(&a))
        .find(|ord| *ord != Ordering::Equal)
        .unwrap_or(Ordering::Equal)
    }
  });
  // dbg!(&plays);

  Some(plays.iter().enumerate().map(|(i, play)| (i as u64 + 1) * play.bid).sum())
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
    assert_eq!(result, Some(248750248));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(5905));
  }
}
