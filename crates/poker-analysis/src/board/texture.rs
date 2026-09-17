use poker_core::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardTexture {
    pub is_paired: bool,
    pub is_monotone: bool,
    pub is_two_tone: bool,
    pub is_rainbow: bool,
    pub max_straight_possible: usize,
}

pub fn analyze_board(board: &[Card]) -> BoardTexture {
    let mut ranks = [0u8; 13];
    let mut suits = [0u8; 4];

    for card in board {
        ranks[card.rank().index() as usize] += 1;
        suits[card.suit().index() as usize] += 1;
    }

    let is_paired = ranks.iter().any(|&c| c >= 2);
    let mut num_suits = 0;
    let mut max_suit_count = 0;
    for &count in &suits {
        if count > 0 {
            num_suits += 1;
        }
        if count > max_suit_count {
            max_suit_count = count;
        }
    }

    let _is_monotone = max_suit_count >= 3 && num_suits == 1; // Or max_suit_count >= 3 on flop.
                                                              // Technically monotone means all cards same suit. Let's say all same suit.
    let is_monotone = max_suit_count == board.len() as u8 && board.len() >= 3;
    let is_two_tone = max_suit_count == 2 && board.len() >= 3;
    let is_rainbow = max_suit_count == 1 && board.len() >= 3;

    // Check straight possible
    let mut max_straight_possible = 0;

    // Wheel check A-2-3-4-5
    let mut wheel_count = 0;
    if ranks[12] > 0 {
        wheel_count += 1;
    }
    for &r in &ranks[..4] {
        if r > 0 {
            wheel_count += 1;
        }
    }
    max_straight_possible = max_straight_possible.max(wheel_count);

    for start in 0..=8 {
        let mut count = 0;
        for &r in &ranks[start..start + 5] {
            if r > 0 {
                count += 1;
            }
        }
        if count > max_straight_possible {
            max_straight_possible = count;
        }
    }

    BoardTexture {
        is_paired,
        is_monotone,
        is_two_tone,
        is_rainbow,
        max_straight_possible: max_straight_possible as usize,
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
    fn test_board_texture() {
        let board = parse_cards("2h 7h 9d");
        let tex = analyze_board(&board);
        assert!(!tex.is_paired);
        assert!(tex.is_two_tone);
        assert!(!tex.is_monotone);
        assert!(!tex.is_rainbow);
    }

    #[test]
    fn test_board_texture_monotone() {
        let board = parse_cards("2h 7h 9h");
        let tex = analyze_board(&board);
        assert!(tex.is_monotone);
    }
}
