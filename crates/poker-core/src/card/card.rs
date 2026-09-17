use super::{Rank, Suit};
use crate::error::PokerError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card(u8);

impl Card {
    pub const ALL: [Card; 52] = {
        let mut cards = [Card(0); 52];
        let mut i = 0;
        while i < 52 {
            cards[i] = Card(i as u8);
            i += 1;
        }
        cards
    };

    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self(rank.index() * 4 + suit.index())
    }

    pub fn from_index(index: u8) -> Result<Self, PokerError> {
        if index < 52 {
            Ok(Self(index))
        } else {
            Err(PokerError::InvalidCard { index })
        }
    }

    pub fn index(&self) -> u8 {
        self.0
    }

    pub fn rank(&self) -> Rank {
        Rank::from_index(self.0 / 4).unwrap()
    }

    pub fn suit(&self) -> Suit {
        Suit::from_index(self.0 % 4).unwrap()
    }

    pub fn to_bit_mask(&self) -> u64 {
        1u64 << self.0
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, PokerError> {
        <Self as FromStr>::from_str(s)
    }
}

impl FromStr for Card {
    type Err = PokerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(PokerError::InvalidCard { index: 255 });
        }
        let rank = Rank::from_char(chars[0]).ok_or(PokerError::InvalidCard { index: 255 })?;
        let suit = Suit::from_char(chars[1]).ok_or(PokerError::InvalidCard { index: 255 })?;
        Ok(Self::new(rank, suit))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let c = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(c.rank(), Rank::Ace);
        assert_eq!(c.suit(), Suit::Spades);
    }

    #[test]
    fn test_from_str() {
        let c = Card::from_str("Ah").unwrap();
        assert_eq!(c.rank(), Rank::Ace);
        assert_eq!(c.suit(), Suit::Hearts);
    }

    #[test]
    fn test_bit_mask() {
        let c = Card::new(Rank::Two, Suit::Clubs); // index 0
        assert_eq!(c.to_bit_mask(), 1);
    }
}
