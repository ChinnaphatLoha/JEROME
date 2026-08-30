use poker_core::ActionType;

/// Structure holding EV calculation for a specific action.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionEV {
    /// The action considered.
    pub action: ActionType,
    /// Human-readable label for the action.
    pub label: String,
    /// Calculated Expected Value.
    pub ev: f64,
}

/// Calculates Expected Value for a given action.
///
/// `equity`: Hero's equity (0.0 to 1.0)
/// `pot`: Current pot size
/// `to_call`: Amount hero needs to call (if any)
/// `fold_equity`: Estimated probability that opponent will fold (0.0 to 1.0)
pub fn calculate_ev(
    action: &ActionType,
    equity: f64,
    pot: u64,
    to_call: u64,
    fold_equity: f64,
) -> f64 {
    match action {
        ActionType::Fold => 0.0,
        ActionType::Check => equity * pot as f64,
        ActionType::Call => {
            // EV of calling = (Equity * (Pot + ToCall)) - ((1 - Equity) * ToCall)
            // Note: the new pot will be Pot + ToCall + Opponent's matched call amount (assuming they bet to_call and we call).
            // Actually, if pot already includes opponent's bet, we are risking `to_call` to win `pot`.
            let win_amount = pot as f64;
            let lose_amount = to_call as f64;
            (equity * win_amount) - ((1.0 - equity) * lose_amount)
        }
        ActionType::Bet(amount) | ActionType::Raise(amount) | ActionType::AllIn(amount) => {
            let amount = *amount as f64;
            // Fold EV: we win the current pot
            let fold_ev = fold_equity * pot as f64;

            // Call EV: we get called (1 - fold_equity)
            // We risk `amount`, and if we win, we win `pot + amount`
            let call_ev = (1.0 - fold_equity)
                * ((equity * (pot as f64 + amount)) - ((1.0 - equity) * amount));

            fold_ev + call_ev
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_ev_fold() {
        assert_eq!(calculate_ev(&ActionType::Fold, 0.5, 100, 10, 0.0), 0.0);
    }

    #[test]
    fn test_calculate_ev_call() {
        let ev = calculate_ev(&ActionType::Call, 0.5, 100, 50, 0.0);
        // (0.5 * 100) - (0.5 * 50) = 50 - 25 = 25
        assert_eq!(ev, 25.0);
    }

    #[test]
    fn test_calculate_ev_bet() {
        let ev = calculate_ev(&ActionType::Bet(50), 0.5, 100, 0, 0.2);
        // Fold EV = 0.2 * 100 = 20
        // Call EV = 0.8 * ((0.5 * 150) - (0.5 * 50)) = 0.8 * (75 - 25) = 0.8 * 50 = 40
        // Total EV = 20 + 40 = 60
        assert_eq!(ev, 60.0);
    }
}
