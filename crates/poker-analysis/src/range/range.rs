use super::combo::Combo;
use poker_core::card::{Card, Deck};

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    combos: Vec<Combo>,
}

impl Range {
    pub fn empty() -> Self {
        Self { combos: Vec::new() }
    }

    pub fn full() -> Self {
        let mut combos = Vec::with_capacity(1326);
        let deck = Deck::full();
        let cards: Vec<Card> = deck.iter().collect();
        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                combos.push(Combo::new(cards[i], cards[j], 1.0));
            }
        }
        Self { combos }
    }

    pub fn remove_dead_cards(&mut self, dead_deck: &Deck) {
        self.combos.retain(|combo| {
            !dead_deck.contains(combo.cards[0]) && !dead_deck.contains(combo.cards[1])
        });
    }

    pub fn normalize(&mut self) {
        let total = self.total_weight();
        if total > 0.0 {
            for combo in &mut self.combos {
                combo.weight /= total;
            }
        }
    }

    pub fn add_combo(&mut self, combo: Combo) {
        if let Some(existing) = self.combos.iter_mut().find(|c| c.cards == combo.cards) {
            existing.weight = combo.weight;
        } else {
            self.combos.push(combo);
        }
    }

    pub fn remove_combo(&mut self, cards: [Card; 2]) {
        let canonical_cards = if cards[0].index() > cards[1].index() {
            [cards[0], cards[1]]
        } else {
            [cards[1], cards[0]]
        };
        self.combos.retain(|c| c.cards != canonical_cards);
    }

    pub fn modify_weight(&mut self, cards: [Card; 2], weight: f64) {
        let canonical_cards = if cards[0].index() > cards[1].index() {
            [cards[0], cards[1]]
        } else {
            [cards[1], cards[0]]
        };
        if let Some(existing) = self.combos.iter_mut().find(|c| c.cards == canonical_cards) {
            existing.weight = weight;
        }
    }

    pub fn total_weight(&self) -> f64 {
        self.combos.iter().map(|c| c.weight).sum()
    }

    pub fn combos(&self) -> &[Combo] {
        &self.combos
    }

    pub fn combos_mut(&mut self) -> &mut Vec<Combo> {
        &mut self.combos
    }
}

impl IntoIterator for Range {
    type Item = Combo;
    type IntoIter = std::vec::IntoIter<Combo>;

    fn into_iter(self) -> Self::IntoIter {
        self.combos.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_range() {
        let range = Range::full();
        assert_eq!(range.combos().len(), 1326);
    }

    #[test]
    fn test_remove_dead_cards() {
        let mut range = Range::full();
        let mut dead = Deck::empty();
        dead.add(Card::from_str("Ah").unwrap());
        range.remove_dead_cards(&dead);
        assert_eq!(range.combos().len(), 1326 - 51);
    }
}
