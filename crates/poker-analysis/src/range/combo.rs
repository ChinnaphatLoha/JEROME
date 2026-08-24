use poker_core::card::Card;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Combo {
    pub cards: [Card; 2],
    pub weight: f64,
}

impl Combo {
    pub fn new(c1: Card, c2: Card, weight: f64) -> Self {
        // canonicalize order to avoid duplicates (higher card first)
        let (c1, c2) = if c1.index() > c2.index() {
            (c1, c2)
        } else {
            (c2, c1)
        };
        Self {
            cards: [c1, c2],
            weight,
        }
    }

    pub fn has_card(&self, card: Card) -> bool {
        self.cards[0] == card || self.cards[1] == card
    }

    pub fn is_pair(&self) -> bool {
        self.cards[0].rank() == self.cards[1].rank()
    }

    pub fn is_suited(&self) -> bool {
        self.cards[0].suit() == self.cards[1].suit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::card::{Rank, Suit};

    #[test]
    fn test_combo() {
        let c1 = Card::new(Rank::Ace, Suit::Spades);
        let c2 = Card::new(Rank::Ace, Suit::Hearts);
        let combo = Combo::new(c1, c2, 1.0);
        assert!(combo.is_pair());
        assert!(!combo.is_suited());
        assert!(combo.has_card(c1));
    }
}
