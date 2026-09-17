//! JavaScript-facing DTO types for the WASM boundary.
//!
//! These types use `serde` for serialization/deserialization at the adapter boundary.
//! They are intentionally separate from JEROME's core domain types to keep
//! serialization concerns out of the mathematical engine.

use serde::{Deserialize, Serialize};

// ─── Input DTOs ─────────────────────────────────────────────────────

/// JavaScript-facing game state input.
#[derive(Debug, Deserialize)]
pub struct JsGameStateInput {
    /// Hero's hole cards as 2-character strings, e.g. `["As", "Kh"]`.
    pub hero_cards: [String; 2],
    /// Community cards as 2-character strings, e.g. `["Qs", "Th", "5d"]`.
    pub board: Vec<String>,
    /// Current street: `"preflop"`, `"flop"`, `"turn"`, or `"river"`.
    pub street: String,
    /// Current pot size in chips.
    pub pot: u64,
    /// Current bet to act on.
    pub current_bet: u64,
    /// Big blind size in chips.
    pub big_blind: u64,
    /// Minimum raise size (defaults to `big_blind` if omitted).
    pub min_raise: Option<u64>,
    /// Index of the hero in the `players` array.
    pub hero_index: usize,
    /// Players at the table.
    pub players: Vec<JsPlayer>,
    /// Optional engine configuration.
    pub config: Option<JsConfig>,
}

/// JavaScript-facing player input.
#[derive(Debug, Deserialize)]
pub struct JsPlayer {
    /// Player identifier.
    pub id: u8,
    /// Table position: `"UTG"`, `"UTG1"`, `"MP"`, `"MP1"`, `"HJ"`, `"CO"`, `"BTN"`, `"SB"`, `"BB"`.
    pub position: String,
    /// Remaining stack in chips.
    pub stack: u64,
    /// Player status: `"active"`, `"folded"`, `"allin"`, or `"eliminated"`.
    pub status: String,
    /// Amount bet this round.
    pub bet_this_round: u64,
}

/// Optional engine configuration for WASM consumers.
#[derive(Debug, Deserialize)]
pub struct JsConfig {
    /// Number of Monte Carlo samples (default: 10000).
    pub mc_samples: Option<usize>,
    /// RNG seed for deterministic results (default: 42).
    pub seed: Option<u64>,
}

// ─── Output DTOs ────────────────────────────────────────────────────

/// JavaScript-facing decision result.
#[derive(Debug, Serialize)]
pub struct JsDecisionResult {
    /// The recommended action.
    pub recommended_action: JsAction,
    /// Size of the recommended action (if applicable).
    pub recommended_size: Option<u64>,
    /// Estimated equity (0.0–1.0).
    pub estimated_equity: f64,
    /// Minimum required equity for a profitable call.
    pub required_equity: f64,
    /// Expected value of the recommended action.
    pub estimated_ev: f64,
    /// All evaluated alternatives sorted by EV (highest first).
    pub alternatives: Vec<JsActionEV>,
    /// Human-readable explanation of the decision.
    pub explanation: JsExplanation,
}

/// JavaScript-facing action representation.
#[derive(Debug, Serialize)]
pub struct JsAction {
    /// Action type: `"Fold"`, `"Check"`, `"Call"`, `"Bet"`, `"Raise"`, `"AllIn"`.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Amount for sized actions (Bet, Raise, AllIn). `None` for Fold/Check/Call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
}

/// JavaScript-facing action with expected value.
#[derive(Debug, Serialize)]
pub struct JsActionEV {
    /// The action considered.
    pub action: JsAction,
    /// Human-readable label for the action.
    pub label: String,
    /// Calculated expected value.
    pub ev: f64,
}

/// JavaScript-facing decision explanation.
#[derive(Debug, Serialize)]
pub struct JsExplanation {
    /// Contributing factors and their descriptions.
    pub factors: Vec<JsFactor>,
    /// Label of the recommended action.
    pub recommended_action_label: String,
}

/// A single decision factor.
#[derive(Debug, Serialize)]
pub struct JsFactor {
    /// Factor name (e.g. `"StrongEquity"`, `"PotOdds"`).
    pub factor: String,
    /// Human-readable description.
    pub description: String,
}
