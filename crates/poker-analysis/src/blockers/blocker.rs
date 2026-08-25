use crate::range::Range;
use poker_core::card::Deck;

pub struct BlockerAnalyzer;

impl BlockerAnalyzer {
    pub fn apply_blockers(range: &mut Range, known_cards: &Deck) {
        range.remove_dead_cards(known_cards);
    }
}

pub fn apply_blockers(range: &mut Range, known_cards: &Deck) {
    BlockerAnalyzer::apply_blockers(range, known_cards);
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::card::Card;

    #[test]
    fn test_apply_blockers() {
        let mut range = Range::full();
        let mut known = Deck::empty();
        known.add(Card::from_str("Ah").unwrap());
        known.add(Card::from_str("Kh").unwrap());

        apply_blockers(&mut range, &known);
        // 1326 total combos.
        // Combinations with Ah: 51
        // Combinations with Kh: 51
        // Combinations with both: 1
        // Total blocked: 51 + 51 - 1 = 101
        // Remaining: 1326 - 101 = 1225
        assert_eq!(range.combos().len(), 1225);
    }
}
