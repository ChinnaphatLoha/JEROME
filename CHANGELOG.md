# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
