/// Determines if a bluff might be profitable.
pub fn should_bluff(fold_equity: f64, equity: f64, pot: u64, bet_size: u64) -> bool {
    let ev = bluff_ev(fold_equity, equity, pot, bet_size);
    ev > 0.0
}

/// Calculates the EV of a bluff or semi-bluff.
pub fn bluff_ev(fold_equity: f64, equity: f64, pot: u64, bet_size: u64) -> f64 {
    let win_amount = pot as f64;
    let lose_amount = bet_size as f64;
    
    // EV = (Fold% * WinAmount) + ((1-Fold%) * ((Equity * (WinAmount + LoseAmount)) - ((1-Equity) * LoseAmount)))
    let fold_ev = fold_equity * win_amount;
    let call_ev = (1.0 - fold_equity) * (
        (equity * (win_amount + lose_amount)) - ((1.0 - equity) * lose_amount)
    );
    
    fold_ev + call_ev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluff_ev() {
        // Pure bluff: 0 equity, 50% fold equity, bet 50 into 100
        // Fold EV = 0.5 * 100 = 50
        // Call EV = 0.5 * (0 - 1.0 * 50) = -25
        // Total = 25
        let ev = bluff_ev(0.5, 0.0, 100, 50);
        assert_eq!(ev, 25.0);
        assert!(should_bluff(0.5, 0.0, 100, 50));
    }
}
