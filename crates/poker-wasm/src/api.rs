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
/// Throws a string error if the input cannot be parsed or if the game state is invalid.
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
