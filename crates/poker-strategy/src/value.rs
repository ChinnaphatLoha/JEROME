/// Determines whether a value bet should be made based on equity and SPR (Stack-to-Pot Ratio).
pub fn should_value_bet(equity: f64, spr: f64) -> bool {
    // Value bet if equity is very high, or if equity is decent and SPR is low
    equity > 0.6 || (equity > 0.5 && spr < 3.0)
}

/// Suggests a sizing for a value bet.
pub fn value_bet_sizing(equity: f64, pot: u64, spr: f64) -> u64 {
    if equity > 0.8 {
        // Very strong hand, bet big or go all in if SPR is low
        if spr < 1.0 {
            (pot as f64 * spr).round() as u64 // all-in essentially
        } else {
            (pot as f64 * 0.75).round() as u64
        }
    } else if equity > 0.6 {
        // Good hand, standard value bet
        (pot as f64 * 0.66).round() as u64
    } else {
        // Thin value bet
        (pot as f64 * 0.5).round() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_value_bet() {
        assert!(should_value_bet(0.65, 5.0));
        assert!(!should_value_bet(0.55, 5.0));
        assert!(should_value_bet(0.55, 2.0));
    }
}
