use std::cmp::Ordering;

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

fn get_inner_hand<T: TryFrom<char>>(hand: &str) -> Option<[T; 5]> {
  let mut cards = hand.chars().filter_map(|c| T::try_from(c).ok());
  let (Some(c1), Some(c2), Some(c3), Some(c4), Some(c5)) = (cards.next(), cards.next(), cards.next(), cards.next(), cards.next()) else {
    unimplemented!("Invalid hand: {hand}");
  };
  Some([c1, c2, c3, c4, c5])
}

pub fn part_one(input: &str) -> Option<u64> {
  let mut plays: Vec<Play> = input
    .lines()
    .filter_map(|l| {
      let mut parts = l.split_whitespace();
      let (Some(hand), Some(bid)) = (parts.next(), parts.next()) else {
        return None;
      };

      let inner = get_inner_hand(hand)?;
      let hand = Hand::new(inner);
      let bid = bid.parse().unwrap();

      let play = Play { hand, bid };
      Some(play)
    })
    .collect();

  plays.sort_by(|a, b| b.hand.cmp(&a.hand));

  Some(plays.iter().enumerate().map(|(i, play)| (i as u64 + 1) * play.bid).sum())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Play2 {
  hand: Hand2,
  bid: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card2 {
  _A,
  _K,
  _Q,
  _T,
  _9,
  _8,
  _7,
  _6,
  _5,
  _4,
  _3,
  _2,
  _J,
}

impl TryFrom<char> for Card2 {
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

type InnerHand2 = [Card2; 5];
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord)]
struct Hand2 {
  strength: Strength,
  inner: InnerHand2,
}

impl Hand2 {
  fn new(inner: InnerHand2) -> Self {
    let mut counts = [0_usize; 13];
    let mut j_count = 0;
    for card in inner.iter() {
      if *card == Card2::_J {
        j_count += 1;
      } else {
        counts[*card as usize] += 1;
      }
    }

    counts.sort_unstable_by(|a, b| b.cmp(a));

    let tmp_strength = match counts {
      [5, ..] => Strength::FiveOfAKind,
      [4, ..] => Strength::FourOfAKind,
      [3, 2, ..] => Strength::FullHouse,
      [3, ..] => Strength::ThreeOfAKind,
      [2, 2, ..] => Strength::TwoPairs,
      [2, ..] => Strength::OnePair,
      _ => Strength::HighCard,
    };

    let strength = match (tmp_strength, j_count) {
      (Strength::FiveOfAKind, _) => Strength::FiveOfAKind,
      (Strength::FourOfAKind, 1..) => Strength::FiveOfAKind,
      (Strength::FullHouse, _) => Strength::FullHouse,
      (Strength::ThreeOfAKind, 1) => Strength::FourOfAKind,
      (Strength::ThreeOfAKind, 2) => Strength::FiveOfAKind,
      (Strength::TwoPairs, 1) => Strength::FullHouse,
      (Strength::OnePair, 1) => Strength::ThreeOfAKind,
      (Strength::OnePair, 2) => Strength::FourOfAKind,
      (Strength::OnePair, 3..) => Strength::FiveOfAKind,
      (Strength::HighCard, 1) => Strength::OnePair,
      (Strength::HighCard, 2) => Strength::ThreeOfAKind,
      (Strength::HighCard, 3) => Strength::FourOfAKind,
      (Strength::HighCard, 4) => Strength::FiveOfAKind,
      _ => tmp_strength,
    };

    Self { strength, inner }
  }
}

impl PartialOrd for Hand2 {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.strength != other.strength {
      return self.strength.partial_cmp(&other.strength);
    }
    std::iter::zip(self.inner.iter(), other.inner.iter())
      .find_map(|(a, b)| a.partial_cmp(b))
      .or_else(|| Some(Ordering::Equal))
  }
}

pub fn part_two(input: &str) -> Option<u64> {
  let mut plays: Vec<Play2> = input
    .lines()
    .filter_map(|l| {
      let mut parts = l.split_whitespace();
      let (Some(hand), Some(bid)) = (parts.next(), parts.next()) else {
        return None;
      };

      let inner = get_inner_hand(hand)?;
      let hand = Hand2::new(inner);
      let bid = bid.parse().unwrap();

      let play = Play2 { hand, bid };
      Some(play)
    })
    .collect();

  plays.sort_by(|a, b| b.hand.cmp(&a.hand));

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
    assert_eq!(result, Some(248642943));
  }

  #[test]
  fn test_part_two_example() {
    let result = part_two(EXAMPLE);
    assert_eq!(result, Some(5905));
  }
}
