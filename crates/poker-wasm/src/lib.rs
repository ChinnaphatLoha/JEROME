//! # poker-wasm
//!
//! WebAssembly adapter for the JEROME poker decision engine.
//!
//! This crate provides a thin interoperability layer that allows JavaScript/TypeScript
//! consumers to use JEROME's deterministic poker analysis engine via WebAssembly.
//!
//! The adapter accepts plain JavaScript values, validates and converts them into
//! JEROME's strongly-typed structures, delegates to the existing engine, and returns
//! JavaScript-compatible structured results.
//!
//! # Architecture
//!
//! ```text
//! JavaScript input (JsValue)
//!     ↓
//! Boundary validation / conversion (types.rs, convert.rs)
//!     ↓
//! PokerEngine::analyze() (poker-engine)
//!     ↓
//! Boundary conversion (convert.rs)
//!     ↓
//! JavaScript output (JsValue)
//! ```

mod api;
mod convert;
mod error;
mod types;

pub use api::analyze;
