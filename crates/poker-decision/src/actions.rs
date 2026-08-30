use crate::config::ActionAbstractionConfig;
use poker_core::{ActionType, GameState};

/// Represents a candidate action and its human-readable label.
#[derive(Debug, Clone, PartialEq)]
pub struct CandidateAction {
    /// The action itself.
    pub action: ActionType,
    /// A human-readable label for the action (e.g., "Pot size bet", "Min raise").
    pub label: String,
}

/// Generates a list of valid candidate actions given a game state and configuration.
pub fn generate_candidates(
    state: &GameState,
    config: &ActionAbstractionConfig,
) -> Vec<CandidateAction> {
    let mut candidates = Vec::new();
    let hero = state.hero();
    let stack = hero.stack;
    let to_call = state.to_call();
    let min_raise = state.min_raise;
    let current_bet = state.current_bet;
    let pot = state.pot;

    // Always can fold if facing a bet
    if to_call > 0 {
        candidates.push(CandidateAction {
            action: ActionType::Fold,
            label: "Fold".to_string(),
        });
    }

    // Check if no bet to call
    if to_call == 0 {
        candidates.push(CandidateAction {
            action: ActionType::Check,
            label: "Check".to_string(),
        });
    }

    // Call if there's a bet and we have chips
    if to_call > 0 && stack > 0 {
        let _call_amount = to_call.min(stack);
        candidates.push(CandidateAction {
            action: ActionType::Call,
            label: "Call".to_string(),
        });
    }

    // Betting options (if no one has bet yet)
    if to_call == 0 && stack > 0 {
        for &size in &config.bet_sizes {
            let mut bet_amount = (pot as f64 * size).round() as u64;
            bet_amount = bet_amount.max(min_raise);
            if bet_amount < stack {
                candidates.push(CandidateAction {
                    action: ActionType::Bet(bet_amount),
                    label: format!("Bet {:.0}% Pot", size * 100.0),
                });
            }
        }
    }

    // Raising options (if someone has bet)
    if to_call > 0 && stack > to_call {
        for &size in &config.raise_sizes {
            let raise_amount = (current_bet as f64 * size).round() as u64;
            let total_raise = raise_amount.max(current_bet + min_raise);
            if total_raise < stack + current_bet {
                let actual_raise = total_raise - current_bet;
                candidates.push(CandidateAction {
                    action: ActionType::Raise(actual_raise),
                    label: format!("Raise {:.1}x", size),
                });
            }
        }
    }

    // Always option to All-in if we have stack
    if stack > 0 {
        // Find if this is a bet, raise, or call technically
        if stack <= to_call {
            candidates.push(CandidateAction {
                action: ActionType::Call, // Technically an all-in call
                label: "All-In".to_string(),
            });
        } else {
            candidates.push(CandidateAction {
                action: ActionType::AllIn(stack),
                label: "All-In".to_string(),
            });
        }
    }

    candidates
}

#[cfg(test)]
mod tests {
    // Tests module initialized
}
