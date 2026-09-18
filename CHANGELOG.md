# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **`poker-wasm` Public Readiness**: Comprehensive developer-facing documentation (`crates/poker-wasm/README.md`) covering installation, input/output schemas, error handling, deterministic execution, TypeScript usage, and browser/Node.js support.
- **TypeScript Type Declarations**: Hand-written `.d.ts` with full types for `AnalysisInput`, `AnalysisResult`, `Action`, `ActionEV`, `Explanation`, `Factor`, `AnalysisError`, and all string literal types — replacing the auto-generated `any`/`any` signatures.
- **Structured WASM Error Objects**: Errors now throw JavaScript `Error` objects with a machine-readable `code` property (`INVALID_CARD`, `INVALID_ENUM`, `DESERIALIZATION_ERROR`, `ENGINE_ERROR`) for programmatic error handling.
- **WASM Integration Tests**: `wasm-bindgen-test` integration tests exercising the actual `#[wasm_bindgen]` boundary — happy paths for all 4 streets, determinism verification, error code validation, and output structure checks.
- **Expanded Unit Tests**: 25 additional unit tests covering turn/river analysis, all-in status variants, multi-player scenarios, edge cases (hero_index out of range, board/hero card overlap), error message specificity, case insensitivity, and output structure completeness (39 total, up from 14).
- **TypeScript Example** (`example.ts`): Demonstrates fully typed usage with `AnalysisInput`, `AnalysisResult`, and `AnalysisError`.
- **CI: WASM Build & Verification**: WASM CI job now runs `wasm-pack build`, verifies generated package structure, and runs unit tests (previously only ran `cargo check`).

### Changed
- **npm Package Name**: Renamed from `poker-wasm` to `jerome-poker-wasm` for npm uniqueness and consistent JEROME branding.
- **`poker-wasm` Cargo.toml**: Added `repository`, `homepage`, `keywords`, and `categories` metadata for public distribution readiness.
- **WASM Error Handling**: Errors thrown across the WASM boundary are now JavaScript `Error` objects instead of plain strings.
- **Root README WebAssembly Section**: Updated to match actual package name (`jerome-poker-wasm`), added build instructions, and linked to dedicated WASM documentation.
- **`example.js`**: Rewritten to use the `web` target API pattern (`init()` + `analyze()`) with error handling demonstration.
- **Root README**: Added `poker-wasm` to workspace crates table.
- **$O(1)$ Hand Evaluator (`poker-analysis`)**: Constant-time 7-card evaluator using prime multiplication and bitmasking lookup tables, cutting evaluation latency to ~36 ns (~90% speedup).
- **Multi-Way Equity Engine (`poker-probability`)**: Rejection-sampled Monte Carlo simulation for multi-way ($N$-player) pot equity calculation with card collision avoidance.
- **Counterfactual Regret Minimization (`poker-strategy`)**: CFR solver engine for computing Nash equilibria and GTO approximations in abstracted two-player zero-sum subgames (validated on Kuhn poker).
- **Position-Aware Preflop Range Charts (`poker-analysis`)**: Built-in RFI (Raise First In) opening ranges for 6-max and 9-max tables with position-scaled frequencies (UTG ~10% to BTN ~45%).

### Changed
- **Criterion Benchmarks**: Updated benchmark suite (`benches/poker_benchmarks.rs`) to profile live evaluator workloads and accurate per-iteration equity calculation overhead.
- **Documentation Overhaul**: Modernized `README.md` with architectural dataflow diagrams, updated benchmark tables, roadmap progress, and typography refinements.
- **Project Rebranding**: Renamed project to JEROME (*Judgement Engine for Range, Odds, Moves & Equity*) with updated documentation and crate paths.

### Fixed
- **Code Quality & Clippy**: Cleaned up codebase across all 7 workspace crates to strictly satisfy all Clippy lints (improved borrowing, casting, and dead code cleanup).

## [0.1.0] - 2026-09-12

### Added
- Workspace architecture with 7 specialized crates
- Card, deck, game state, and betting primitives (`poker-core`)
- Hand evaluation, board texture analysis, and range modeling (`poker-analysis`)
- Monte Carlo and exact equity calculation (`poker-probability`)
- EV-based action generation and decision pipeline (`poker-decision`)
- GTO approximations, value betting, and bluff strategy (`poker-strategy`)
- Scenario simulation framework (`poker-simulation`)
- Unified `PokerEngine` facade with builder-pattern game state
- Criterion benchmarks for hand evaluation and full decision pipeline
- Integration test suite
