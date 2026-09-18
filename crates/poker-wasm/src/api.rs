//! The JavaScript/WASM API boundary.

use crate::convert::{from_decision_result, to_decision_config, to_game_state};
use crate::error::WasmError;
use crate::types::JsGameStateInput;
use poker_engine::{EngineConfig, PokerEngine};
use wasm_bindgen::prelude::*;

/// Analyzes a poker game state and returns an optimal decision.
///
/// # Arguments
/// * `input` - A JavaScript object conforming to the `JsGameStateInput` structure.
///
/// # Returns
/// A JavaScript object conforming to the `JsDecisionResult` structure.
///
/// # Errors
/// Throws a JavaScript `Error` with a `code` property for programmatic handling:
/// - `DESERIALIZATION_ERROR` — input could not be parsed from JavaScript
/// - `INVALID_CARD` — a card string like `"Xx"` is not a valid card
/// - `INVALID_ENUM` — a string field like `street` or `position` has an unrecognized value
/// - `ENGINE_ERROR` — the game state is invalid or the engine encountered an error
#[wasm_bindgen]
pub fn analyze(input: JsValue) -> Result<JsValue, JsValue> {
    // 1. Parse JavaScript object to DTO
    let input_dto: JsGameStateInput = serde_wasm_bindgen::from_value(input)
        .map_err(|e| WasmError::DeserializationError(e.to_string()))?;

    // 2. Convert DTO to JEROME native types
    let game_state = to_game_state(&input_dto)?;
    let decision_config = to_decision_config(input_dto.config.as_ref());

    // 3. Initialize engine with the config
    let engine_config = EngineConfig { decision_config };
    let engine = PokerEngine::new(engine_config);

    // 4. Run the decision engine
    let decision_result = engine
        .analyze(&game_state)
        .map_err(WasmError::from_poker_error)?;

    // 5. Convert result back to DTO
    let output_dto = from_decision_result(&decision_result);

    // 6. Serialize DTO to JavaScript object
    serde_wasm_bindgen::to_value(&output_dto)
        .map_err(|e| WasmError::EngineError(format!("Failed to serialize result: {}", e)).into())
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
// ─── Input Types ────────────────────────────────────────────────────

/** Card string: rank + suit (e.g. `"As"`, `"Th"`, `"2c"`). */
export type Card = string;

/** Betting street. */
export type Street = "preflop" | "flop" | "turn" | "river";

/** Table position. */
export type Position =
  | "UTG"
  | "UTG1"
  | "MP"
  | "MP1"
  | "HJ"
  | "CO"
  | "BTN"
  | "SB"
  | "BB";

/** Player status. */
export type PlayerStatus =
  | "active"
  | "folded"
  | "allin"
  | "all_in"
  | "all-in"
  | "eliminated";

/** A player at the table. */
export interface PlayerInput {
  /** Player identifier (0–255). */
  id: number;
  /** Table position. */
  position: Position;
  /** Remaining stack in chips. */
  stack: number;
  /** Player status. */
  status: PlayerStatus;
  /** Amount bet this round. */
  bet_this_round: number;
}

/** Engine configuration options. */
export interface AnalysisConfig {
  /** Number of Monte Carlo samples (default: 10000). */
  mc_samples?: number;
  /** RNG seed for deterministic results (default: 42). */
  seed?: number;
}

/** Input to the `analyze()` function. */
export interface AnalysisInput {
  hero_cards: [Card, Card];
  board: Card[];
  street: Street;
  pot: number;
  current_bet: number;
  big_blind: number;
  min_raise?: number;
  hero_index: number;
  players: PlayerInput[];
  config?: AnalysisConfig;
}

// ─── Output Types ───────────────────────────────────────────────────

export type ActionType = "Fold" | "Check" | "Call" | "Bet" | "Raise" | "AllIn";

export interface Action {
  type: ActionType;
  amount?: number;
}

export interface ActionEV {
  action: Action;
  label: string;
  ev: number;
}

export interface Factor {
  factor: string;
  description: string;
}

export interface Explanation {
  factors: Factor[];
  recommended_action_label: string;
}

export interface AnalysisResult {
  recommended_action: Action;
  recommended_size: number | null;
  estimated_equity: number;
  required_equity: number;
  estimated_ev: number;
  alternatives: ActionEV[];
  explanation: Explanation;
}

// ─── Error Types ────────────────────────────────────────────────────

export type AnalysisErrorCode =
  | "INVALID_CARD"
  | "INVALID_ENUM"
  | "DESERIALIZATION_ERROR"
  | "ENGINE_ERROR";

export interface AnalysisError extends Error {
  code: AnalysisErrorCode;
}
"#;

