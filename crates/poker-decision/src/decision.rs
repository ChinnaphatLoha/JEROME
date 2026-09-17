use crate::actions::generate_candidates;
use crate::config::DecisionConfig;
use crate::ev::{calculate_ev, ActionEV};
use crate::explanation::{DecisionExplanation, DecisionFactor};
use poker_analysis::board::texture::analyze_board;
use poker_analysis::range::range::Range;
use poker_core::{ActionType, Deck, GameState, PokerError};
use poker_probability::equity::calculate_mc_equity;

/// The final result of a decision analysis.
#[derive(Debug, Clone)]
pub struct DecisionResult {
    /// The recommended action to take.
    pub recommended_action: ActionType,
    /// The size of the action, if applicable (bet, raise, all-in).
    pub recommended_size: Option<u64>,
    /// Estimated equity of the hero's hand.
    pub estimated_equity: f64,
    /// Minimum required equity to make a profitable call based on pot odds.
    pub required_equity: f64,
    /// Expected value of the recommended action.
    pub estimated_ev: f64,
    /// Alternative actions evaluated, sorted by EV (highest first).
    pub alternatives: Vec<ActionEV>,
    /// Explanation of the decision.
    pub explanation: DecisionExplanation,
}

/// Engine responsible for analyzing the game state and making decisions.
pub struct DecisionEngine {
    config: DecisionConfig,
}

impl DecisionEngine {
    pub fn new(config: DecisionConfig) -> Self {
        Self { config }
    }

    /// Analyzes the game state and returns a recommended decision.
    pub fn analyze(&self, state: &GameState) -> Result<DecisionResult, PokerError> {
        state.validate()?;

        // Analyze board
        let _board_texture = analyze_board(&state.board);

        // Calculate required equity (Pot Odds)
        let to_call = state.to_call();
        let pot = state.pot;
        let required_equity = if to_call > 0 {
            to_call as f64 / (pot as f64 + to_call as f64)
        } else {
            0.0
        };

        // Estimate Equity using MC (simplified logic)
        let hero_cards = state.hero_cards;

        let mut op_range = Range::full();
        let mut dead_deck = Deck::empty();
        for &card in &state.board {
            dead_deck.add(card);
        }
        for &card in &hero_cards {
            dead_deck.add(card);
        }
        for &card in &state.dead_cards {
            dead_deck.add(card);
        }
        op_range.remove_dead_cards(&dead_deck);

        // Ensure we actually have 2 cards
        let hero_array = if hero_cards.len() >= 2 {
            [hero_cards[0], hero_cards[1]]
        } else {
            // Unlikely in holdem unless state is bad
            return Err(PokerError::InvalidGameState {
                reason: "Hero must have 2 cards",
            });
        };

        let equity_result = calculate_mc_equity(
            hero_array,
            &op_range,
            &state.board,
            &dead_deck,
            &self.config.equity_config,
        )?;
        let estimated_equity = equity_result.equity;

        // Generate Candidates
        let candidates = generate_candidates(state, &self.config.action_config);

        // Calculate EV for each candidate
        let mut action_evs: Vec<ActionEV> = candidates
            .into_iter()
            .map(|c| {
                let fold_equity = match &c.action {
                    ActionType::Bet(amount)
                    | ActionType::Raise(amount)
                    | ActionType::AllIn(amount) => {
                        let fe = (*amount as f64) / (pot as f64 + *amount as f64);
                        fe.min(0.7)
                    }
                    _ => 0.0,
                };

                let ev = calculate_ev(&c.action, estimated_equity, pot, to_call, fold_equity);
                ActionEV {
                    action: c.action,
                    label: c.label,
                    ev,
                }
            })
            .collect();

        // Rank by EV
        action_evs.sort_by(|a, b| b.ev.partial_cmp(&a.ev).unwrap_or(std::cmp::Ordering::Equal));

        let best_action = action_evs.first().cloned().unwrap_or(ActionEV {
            action: ActionType::Fold,
            label: "Fold".to_string(),
            ev: 0.0,
        });

        // Build Explanation
        let mut explanation = DecisionExplanation::new();
        explanation.recommended_action_label = best_action.label.clone();

        if estimated_equity > 0.6 {
            explanation.add_factor(
                DecisionFactor::StrongEquity,
                format!("Hand has strong equity ({:.1}%)", estimated_equity * 100.0),
            );
        } else if estimated_equity < 0.4 {
            explanation.add_factor(
                DecisionFactor::WeakEquity,
                format!("Hand has weak equity ({:.1}%)", estimated_equity * 100.0),
            );
        }

        if best_action.ev > 0.0 && required_equity > 0.0 && estimated_equity > required_equity {
            explanation.add_factor(
                DecisionFactor::PotOdds,
                format!(
                    "Good pot odds for call: required {:.1}%, actual {:.1}%",
                    required_equity * 100.0,
                    estimated_equity * 100.0
                ),
            );
        }

        let rec_size = match best_action.action {
            ActionType::Bet(amount) | ActionType::Raise(amount) | ActionType::AllIn(amount) => {
                Some(amount)
            }
            _ => None,
        };

        Ok(DecisionResult {
            recommended_action: best_action.action,
            recommended_size: rec_size,
            estimated_equity,
            required_equity,
            estimated_ev: best_action.ev,
            alternatives: action_evs,
            explanation,
        })
    }
}
