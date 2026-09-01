use poker_core::{ActionType, GameState};
use poker_analysis::board::texture::BoardTexture;

/// Represents an adjustment to the base strategy.
#[derive(Debug, Clone, PartialEq)]
pub struct StrategyAdjustment {
    pub action: ActionType,
    /// Multiplier to apply to the probability/weight of this action.
    pub weight_multiplier: f64,
    /// Human-readable reason for the adjustment.
    pub reason: String,
}

/// Engine for generating strategy adjustments based on game state, equity, and board texture.
pub trait StrategyEngine {
    fn adjust(&self, state: &GameState, equity: f64, board: &BoardTexture) -> Vec<StrategyAdjustment>;
}

/// A default implementation of the StrategyEngine using basic heuristics.
pub struct DefaultStrategy;

impl StrategyEngine for DefaultStrategy {
    fn adjust(&self, state: &GameState, equity: f64, board: &BoardTexture) -> Vec<StrategyAdjustment> {
        let mut adjustments = Vec::new();
        
        let _hero = state.hero();
        let to_call = state.to_call();

        if equity > 0.7 {
            // Very strong hand
            adjustments.push(StrategyAdjustment {
                action: ActionType::Raise(0), // Placeholder amount, the decision engine handles sizes
                weight_multiplier: 2.0,
                reason: "Premium equity".to_string(),
            });
        } else if equity < 0.3 && to_call > 0 {
            // Weak hand facing a bet
            adjustments.push(StrategyAdjustment {
                action: ActionType::Fold,
                weight_multiplier: 2.0,
                reason: "Poor equity".to_string(),
            });
        }

        // Adjust based on board texture (example heuristics)
        if board.is_paired || board.is_monotone || board.is_two_tone {
            if equity > 0.5 {
                // Protect vulnerable made hands
                adjustments.push(StrategyAdjustment {
                    action: ActionType::Bet(0), 
                    weight_multiplier: 1.5,
                    reason: "Protection on wet board".to_string(),
                });
            }
        } else {
            // Dry board
            if equity > 0.8 {
                // Slow play monsters occasionally
                adjustments.push(StrategyAdjustment {
                    action: ActionType::Check,
                    weight_multiplier: 1.2,
                    reason: "Slow play on dry board".to_string(),
                });
            }
        }

        adjustments
    }
}
