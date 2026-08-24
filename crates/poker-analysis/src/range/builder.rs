use super::combo::Combo;
use super::range::Range;
use poker_core::card::{Card, Rank, Suit};

pub struct RangeBuilder {
    range: Range,
}

impl RangeBuilder {
    pub fn new() -> Self {
        Self {
            range: Range::empty(),
        }
    }

    pub fn full() -> Self {
        Self {
            range: Range::full(),
        }
    }

    pub fn build(self) -> Range {
        self.range
    }

    pub fn add_pair(mut self, rank: Rank, weight: f64) -> Self {
        for s1 in 0..4 {
            for s2 in (s1 + 1)..4 {
                let c1 = Card::new(rank, Suit::from_index(s1).unwrap());
                let c2 = Card::new(rank, Suit::from_index(s2).unwrap());
                self.range.add_combo(Combo::new(c1, c2, weight));
            }
        }
        self
    }

    pub fn add_suited(mut self, r1: Rank, r2: Rank, weight: f64) -> Self {
        if r1 == r2 {
            return self;
        }
        for s in 0..4 {
            let c1 = Card::new(r1, Suit::from_index(s).unwrap());
            let c2 = Card::new(r2, Suit::from_index(s).unwrap());
            self.range.add_combo(Combo::new(c1, c2, weight));
        }
        self
    }

    pub fn add_offsuit(mut self, r1: Rank, r2: Rank, weight: f64) -> Self {
        if r1 == r2 {
            return self;
        }
        for s1 in 0..4 {
            for s2 in 0..4 {
                if s1 != s2 {
                    let c1 = Card::new(r1, Suit::from_index(s1).unwrap());
                    let c2 = Card::new(r2, Suit::from_index(s2).unwrap());
                    self.range.add_combo(Combo::new(c1, c2, weight));
                }
            }
        }
        self
    }
}

impl Default for RangeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pairs() {
        let range = RangeBuilder::new().add_pair(Rank::Ace, 1.0).build();
        assert_eq!(range.combos().len(), 6);
    }

    #[test]
    fn test_builder_suited() {
        let range = RangeBuilder::new()
            .add_suited(Rank::Ace, Rank::King, 1.0)
            .build();
        assert_eq!(range.combos().len(), 4);
    }

    #[test]
    fn test_builder_offsuit() {
        let range = RangeBuilder::new()
            .add_offsuit(Rank::Ace, Rank::King, 1.0)
            .build();
        assert_eq!(range.combos().len(), 12);
    }
}
