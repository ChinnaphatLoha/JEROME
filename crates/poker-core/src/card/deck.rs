use super::Card;
use crate::error::PokerError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Deck(u64);

impl Deck {
    pub const FULL_DECK: u64 = (1u64 << 52) - 1;

    pub fn full() -> Self {
        Self(Self::FULL_DECK)
    }

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn contains(&self, card: Card) -> bool {
        (self.0 & card.to_bit_mask()) != 0
    }

    pub fn remove(&mut self, card: Card) -> Result<(), PokerError> {
        if !self.contains(card) {
            return Err(PokerError::CardNotAvailable { card: card.index() });
        }
        self.0 &= !card.to_bit_mask();
        Ok(())
    }

    pub fn remove_card(&mut self, card: Card) {
        self.0 &= !card.to_bit_mask();
    }

    pub fn add(&mut self, card: Card) {
        self.0 |= card.to_bit_mask();
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn remaining_cards(&self) -> Vec<Card> {
        self.iter().collect()
    }

    pub fn is_available(&self, card: Card) -> bool {
        self.contains(card)
    }

    pub fn remove_all(&mut self, cards: &[Card]) -> Result<(), PokerError> {
        for &card in cards {
            self.remove(card)?;
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        (0..52).filter_map(move |i| {
            let mask = 1u64 << i;
            if (self.0 & mask) != 0 {
                Some(Card::from_index(i as u8).unwrap())
            } else {
                None
            }
        })
    }

    pub fn mask(&self) -> u64 {
        self.0
    }
}

impl From<&[Card]> for Deck {
    fn from(cards: &[Card]) -> Self {
        let mut deck = Deck::empty();
        for &card in cards {
            deck.add(card);
        }
        deck
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_deck_operations() {
        let mut deck = Deck::full();
        assert_eq!(deck.count(), 52);

        let card = Card::new(Rank::Ace, Suit::Spades);
        assert!(deck.contains(card));

        deck.remove(card).unwrap();
        assert_eq!(deck.count(), 51);
        assert!(!deck.contains(card));

        assert!(deck.remove(card).is_err());
    }
}
