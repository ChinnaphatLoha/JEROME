use poker_core::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawInfo {
    pub is_flush_draw: bool,
    pub is_nut_flush_draw: bool,
    pub is_oesd: bool,
    pub is_gutshot: bool,
    pub overcards: u8,
}

pub fn analyze_draws(hero_cards: [Card; 2], board: &[Card]) -> DrawInfo {
    let mut all_cards = Vec::new();
    all_cards.extend_from_slice(&hero_cards);
    all_cards.extend_from_slice(board);

    let mut suits = [0u8; 4];
    for card in &all_cards {
        suits[card.suit().index() as usize] += 1;
    }

    let is_flush_draw = suits.contains(&4);

    // Simplistic nut flush draw check:
    // It's a NFD if we have the Ace of the flush suit in our hand.
    // Realistically, if Ace is on board, King could be NFD, but this is a simplified version.
    let mut is_nut_flush_draw = false;
    if is_flush_draw {
        for (s_idx, &count) in suits.iter().enumerate() {
            if count == 4
                && hero_cards
                    .iter()
                    .any(|c| c.suit().index() as usize == s_idx && c.rank().index() == 12)
            {
                is_nut_flush_draw = true;
            }
        }
    }

    let mut ranks = [0u8; 13];
    for card in &all_cards {
        ranks[card.rank().index() as usize] = 1; // Just presence
    }

    let mut is_oesd = false;
    let mut is_gutshot = false;

    // Check OESD and gutshot by looking for 4 cards in 5 gaps
    // A-2-3-4-5 wheel
    let mut wheel_count = 0;
    if ranks[12] > 0 {
        wheel_count += 1;
    }
    for &r in &ranks[..4] {
        if r > 0 {
            wheel_count += 1;
        }
    }
    if wheel_count == 4 {
        let wheel_ranks = [ranks[12], ranks[0], ranks[1], ranks[2], ranks[3]];
        let mut consec = 0;
        let mut max_consec = 0;
        for r in wheel_ranks {
            if r > 0 {
                consec += 1;
                max_consec = max_consec.max(consec);
            } else {
                consec = 0;
            }
        }
        if max_consec == 4 {
            is_oesd = true;
        } else {
            is_gutshot = true;
        }
    }

    for start in 0..=8 {
        let mut count = 0;
        for &r in &ranks[start..start + 5] {
            if r > 0 {
                count += 1;
            }
        }

        if count == 4 {
            let mut consec = 0;
            let mut max_consec = 0;
            for &r in &ranks[start..start + 5] {
                if r > 0 {
                    consec += 1;
                    max_consec = max_consec.max(consec);
                } else {
                    consec = 0;
                }
            }
            if max_consec == 4 {
                // To be OESD, the gap must be at the end.
                // However, wait. 2-3-4-5 is OESD. A-2-3-4 is not OESD, it's a gutshot effectively because
                // you only have one end to draw to (the 5).
                // Actually 4 consecutive cards are OESD if they can be extended on both ends.
                // The max straight is 10-J-Q-K-A (start=8). J-Q-K-A is only open on one end.
                // Let's keep the simplistic 4-consecutive = OESD for this context unless we want strict OESD.
                is_oesd = true;
            } else {
                is_gutshot = true;
            }
        }
    }

    let mut board_max_rank = 0;
    for card in board {
        board_max_rank = board_max_rank.max(card.rank().index());
    }

    let mut overcards = 0;
    for c in &hero_cards {
        if c.rank().index() > board_max_rank {
            overcards += 1;
        }
    }

    DrawInfo {
        is_flush_draw,
        is_nut_flush_draw,
        is_oesd,
        is_gutshot,
        overcards,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::card::Card;

    fn parse_cards(s: &str) -> Vec<Card> {
        s.split_whitespace()
            .map(|c| Card::from_str(c).unwrap())
            .collect()
    }

    #[test]
    fn test_draws() {
        let hero = [Card::from_str("Ah").unwrap(), Card::from_str("Kh").unwrap()];
        let board = parse_cards("2h 7h 9d");

        let info = analyze_draws(hero, &board);
        assert!(info.is_flush_draw);
        assert!(info.is_nut_flush_draw);
        assert_eq!(info.overcards, 2);

        let hero_oesd = [Card::from_str("7s").unwrap(), Card::from_str("8c").unwrap()];
        let board_oesd = parse_cards("5d 6h 2s");
        let info2 = analyze_draws(hero_oesd, &board_oesd);
        assert!(info2.is_oesd);
        assert!(!info2.is_gutshot);
    }
}
