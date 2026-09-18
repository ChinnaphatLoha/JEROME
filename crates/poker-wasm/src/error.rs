//! JavaScript-friendly error types for the WASM boundary.
//!
//! Wraps JEROME's `PokerError` into structured JavaScript `Error` objects
//! with error codes that are meaningful to JavaScript consumers without
//! exposing Rust implementation details.

use poker_core::error::PokerError;
use std::fmt;

/// Error type for the WASM adapter boundary.
#[derive(Debug)]
pub enum WasmError {
    /// A card string could not be parsed.
    InvalidCard(String),
    /// An enum string value is not recognized.
    InvalidEnum { field: &'static str, value: String },
    /// The input data could not be deserialized from JavaScript.
    DeserializationError(String),
    /// An error from the JEROME engine.
    EngineError(String),
}

impl WasmError {
    /// Convert a `PokerError` into a `WasmError`.
    pub fn from_poker_error(err: PokerError) -> Self {
        WasmError::EngineError(err.to_string())
    }

    /// Returns a machine-readable error code for programmatic error handling.
    pub fn code(&self) -> &'static str {
        match self {
            WasmError::InvalidCard(_) => "INVALID_CARD",
            WasmError::InvalidEnum { .. } => "INVALID_ENUM",
            WasmError::DeserializationError(_) => "DESERIALIZATION_ERROR",
            WasmError::EngineError(_) => "ENGINE_ERROR",
        }
    }
}

impl fmt::Display for WasmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmError::InvalidCard(s) => write!(f, "Invalid card: \"{}\"", s),
            WasmError::InvalidEnum { field, value } => {
                write!(f, "Invalid {} value: \"{}\"", field, value)
            }
            WasmError::DeserializationError(msg) => {
                write!(f, "Failed to parse input: {}", msg)
            }
            WasmError::EngineError(msg) => write!(f, "Engine error: {}", msg),
        }
    }
}

impl From<WasmError> for wasm_bindgen::JsValue {
    fn from(err: WasmError) -> Self {
        let js_error = js_sys::Error::new(&err.to_string());
        // Set a `code` property for programmatic error handling in JS:
        //   catch (e) { if (e.code === 'INVALID_CARD') { ... } }
        js_sys::Reflect::set(
            &js_error,
            &wasm_bindgen::JsValue::from_str("code"),
            &wasm_bindgen::JsValue::from_str(err.code()),
        )
        .unwrap_or(false);
        js_error.into()
    }
}
