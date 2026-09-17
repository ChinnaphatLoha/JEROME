//! Conversion between JavaScript DTOs and JEROME typed structures.
//!
//! This module translates between the flat, string-based representations
//! used at the WASM boundary and the strongly-typed domain types used
//! internally by JEROME. No poker logic is duplicated here.

use crate::error::WasmError;
use crate::types::{
    JsAction, JsActionEV, JsConfig, JsDecisionResult, JsExplanation, JsFactor, JsGameStateInput,
};
use poker_core::card::Card;
use poker_core::game::game_state::GameStateBuilder;
use poker_core::game::player::{Player, PlayerStatus};
use poker_core::game::position::Position;
use poker_core::game::street::Street;
use poker_core::game::GameState;
use poker_core::ActionType;
use poker_decision::config::DecisionConfig;
use poker_decision::decision::DecisionResult;
use poker_decision::ev::ActionEV;
use poker_decision::explanation::{DecisionExplanation, DecisionFactor};
use poker_probability::equity::EquityConfig;

/// Convert a JavaScript game state input into a JEROME `GameState`.
pub fn to_game_state(input: &JsGameStateInput) -> Result<GameState, WasmError> {
    let hero_cards = parse_hero_cards(&input.hero_cards)?;
    let board = parse_cards(&input.board)?;
    let street = parse_street(&input.street)?;

    let mut builder = GameStateBuilder::new()
        .street(street)
        .hero_cards(hero_cards)
        .board(board)
        .pot(input.pot)
        .current_bet(input.current_bet)
        .big_blind(input.big_blind)
        .min_raise(input.min_raise.unwrap_or(input.big_blind))
        .hero_index(input.hero_index);

    for js_player in &input.players {
        let player = to_player(js_player)?;
        builder = builder.add_player(player);
    }

    builder.build().map_err(WasmError::from_poker_error)
}

/// Convert optional JS config into a `DecisionConfig`.
pub fn to_decision_config(config: Option<&JsConfig>) -> DecisionConfig {
    let equity_config = match config {
        Some(cfg) => EquityConfig {
            mc_samples: cfg.mc_samples.unwrap_or(10_000),
            seed: Some(cfg.seed.unwrap_or(42)),
            ..EquityConfig::default()
        },
        None => EquityConfig {
            seed: Some(42),
            ..EquityConfig::default()
        },
    };

    DecisionConfig {
        equity_config,
        ..DecisionConfig::default()
    }
}

/// Convert a JEROME `DecisionResult` into a JavaScript-facing DTO.
pub fn from_decision_result(result: &DecisionResult) -> JsDecisionResult {
    JsDecisionResult {
        recommended_action: from_action_type(&result.recommended_action),
        recommended_size: result.recommended_size,
        estimated_equity: result.estimated_equity,
        required_equity: result.required_equity,
        estimated_ev: result.estimated_ev,
        alternatives: result.alternatives.iter().map(from_action_ev).collect(),
        explanation: from_explanation(&result.explanation),
    }
}

// ─── Card Parsing ───────────────────────────────────────────────────

fn parse_hero_cards(cards: &[String; 2]) -> Result<[Card; 2], WasmError> {
    let c0 = parse_card(&cards[0])?;
    let c1 = parse_card(&cards[1])?;
    Ok([c0, c1])
}

fn parse_cards(cards: &[String]) -> Result<Vec<Card>, WasmError> {
    cards.iter().map(|s| parse_card(s)).collect()
}

fn parse_card(s: &str) -> Result<Card, WasmError> {
    Card::from_str(s).map_err(|_| WasmError::InvalidCard(s.to_string()))
}

// ─── Enum Parsing ───────────────────────────────────────────────────

fn parse_street(s: &str) -> Result<Street, WasmError> {
    match s.to_lowercase().as_str() {
        "preflop" => Ok(Street::Preflop),
        "flop" => Ok(Street::Flop),
        "turn" => Ok(Street::Turn),
        "river" => Ok(Street::River),
        _ => Err(WasmError::InvalidEnum {
            field: "street",
            value: s.to_string(),
        }),
    }
}

fn parse_position(s: &str) -> Result<Position, WasmError> {
    match s.to_uppercase().as_str() {
        "UTG" => Ok(Position::UTG),
        "UTG1" => Ok(Position::UTG1),
        "MP" => Ok(Position::MP),
        "MP1" => Ok(Position::MP1),
        "HJ" => Ok(Position::HJ),
        "CO" => Ok(Position::CO),
        "BTN" => Ok(Position::BTN),
        "SB" => Ok(Position::SB),
        "BB" => Ok(Position::BB),
        _ => Err(WasmError::InvalidEnum {
            field: "position",
            value: s.to_string(),
        }),
    }
}

fn parse_player_status(s: &str) -> Result<PlayerStatus, WasmError> {
    match s.to_lowercase().as_str() {
        "active" => Ok(PlayerStatus::Active),
        "folded" => Ok(PlayerStatus::Folded),
        "allin" | "all_in" | "all-in" => Ok(PlayerStatus::AllIn),
        "eliminated" => Ok(PlayerStatus::Eliminated),
        _ => Err(WasmError::InvalidEnum {
            field: "status",
            value: s.to_string(),
        }),
    }
}

// ─── Player Conversion ──────────────────────────────────────────────

fn to_player(js: &crate::types::JsPlayer) -> Result<Player, WasmError> {
    Ok(Player {
        id: js.id,
        position: parse_position(&js.position)?,
        stack: js.stack,
        status: parse_player_status(&js.status)?,
        bet_this_round: js.bet_this_round,
    })
}

// ─── Output Conversion ──────────────────────────────────────────────

fn from_action_type(action: &ActionType) -> JsAction {
    match action {
        ActionType::Fold => JsAction {
            action_type: "Fold".to_string(),
            amount: None,
        },
        ActionType::Check => JsAction {
            action_type: "Check".to_string(),
            amount: None,
        },
        ActionType::Call => JsAction {
            action_type: "Call".to_string(),
            amount: None,
        },
        ActionType::Bet(a) => JsAction {
            action_type: "Bet".to_string(),
            amount: Some(*a),
        },
        ActionType::Raise(a) => JsAction {
            action_type: "Raise".to_string(),
            amount: Some(*a),
        },
        ActionType::AllIn(a) => JsAction {
            action_type: "AllIn".to_string(),
            amount: Some(*a),
        },
    }
}

fn from_action_ev(ev: &ActionEV) -> JsActionEV {
    JsActionEV {
        action: from_action_type(&ev.action),
        label: ev.label.clone(),
        ev: ev.ev,
    }
}

fn from_explanation(explanation: &DecisionExplanation) -> JsExplanation {
    JsExplanation {
        factors: explanation.factors.iter().map(from_factor).collect(),
        recommended_action_label: explanation.recommended_action_label.clone(),
    }
}

fn from_factor(factor: &(DecisionFactor, String)) -> JsFactor {
    JsFactor {
        factor: format!("{:?}", factor.0),
        description: factor.1.clone(),
    }
}

// ─── Tests ──────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::JsPlayer;

    fn sample_input() -> JsGameStateInput {
        JsGameStateInput {
            hero_cards: ["As".to_string(), "Ah".to_string()],
            board: vec![],
            street: "preflop".to_string(),
            pot: 3,
            current_bet: 2,
            big_blind: 2,
            min_raise: None,
            hero_index: 0,
            players: vec![
                JsPlayer {
                    id: 0,
                    position: "UTG".to_string(),
                    stack: 998,
                    status: "active".to_string(),
                    bet_this_round: 0,
                },
                JsPlayer {
                    id: 1,
                    position: "BTN".to_string(),
                    stack: 998,
                    status: "active".to_string(),
                    bet_this_round: 0,
                },
            ],
            config: None,
        }
    }

    #[test]
    fn test_to_game_state_valid() {
        let input = sample_input();
        let result = to_game_state(&input);
        assert!(result.is_ok());
        let state = result.unwrap();
        assert_eq!(state.street, Street::Preflop);
        assert_eq!(state.players.len(), 2);
        assert_eq!(state.hero_index, 0);
        assert_eq!(state.pot, 3);
    }

    #[test]
    fn test_to_game_state_flop() {
        let mut input = sample_input();
        input.street = "flop".to_string();
        input.board = vec!["Qs".to_string(), "Th".to_string(), "5d".to_string()];
        input.pot = 120;
        input.current_bet = 40;
        let result = to_game_state(&input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_card() {
        let mut input = sample_input();
        input.hero_cards = ["Xx".to_string(), "Ah".to_string()];
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_cards() {
        let mut input = sample_input();
        input.hero_cards = ["As".to_string(), "As".to_string()];
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_street() {
        let mut input = sample_input();
        input.street = "invalid".to_string();
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_position() {
        let mut input = sample_input();
        input.players[0].position = "INVALID".to_string();
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_status() {
        let mut input = sample_input();
        input.players[0].status = "unknown".to_string();
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_board_count() {
        let mut input = sample_input();
        input.street = "flop".to_string();
        input.board = vec!["Qs".to_string()]; // Only 1 card for flop
        let result = to_game_state(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_decision_config_defaults() {
        let config = to_decision_config(None);
        assert_eq!(config.equity_config.seed, Some(42));
        assert_eq!(config.equity_config.mc_samples, 10_000);
    }

    #[test]
    fn test_decision_config_custom() {
        let js_config = JsConfig {
            mc_samples: Some(5000),
            seed: Some(123),
        };
        let config = to_decision_config(Some(&js_config));
        assert_eq!(config.equity_config.seed, Some(123));
        assert_eq!(config.equity_config.mc_samples, 5000);
    }

    #[test]
    fn test_action_type_conversion() {
        let fold = from_action_type(&ActionType::Fold);
        assert_eq!(fold.action_type, "Fold");
        assert!(fold.amount.is_none());

        let bet = from_action_type(&ActionType::Bet(50));
        assert_eq!(bet.action_type, "Bet");
        assert_eq!(bet.amount, Some(50));

        let allin = from_action_type(&ActionType::AllIn(1000));
        assert_eq!(allin.action_type, "AllIn");
        assert_eq!(allin.amount, Some(1000));
    }

    #[test]
    fn test_full_pipeline_preflop() {
        let input = sample_input();
        let state = to_game_state(&input).unwrap();
        let config = to_decision_config(input.config.as_ref());

        let engine = poker_decision::decision::DecisionEngine::new(config);
        let result = engine.analyze(&state).unwrap();
        let js_result = from_decision_result(&result);

        assert!(js_result.estimated_equity >= 0.0 && js_result.estimated_equity <= 1.0);
        assert!(!js_result.alternatives.is_empty());
    }

    #[test]
    fn test_deterministic_results() {
        let input = sample_input();
        let state = to_game_state(&input).unwrap();
        let config = to_decision_config(input.config.as_ref());

        let engine = poker_decision::decision::DecisionEngine::new(config.clone());
        let r1 = engine.analyze(&state).unwrap();

        let engine2 = poker_decision::decision::DecisionEngine::new(config);
        let r2 = engine2.analyze(&state).unwrap();

        assert_eq!(r1.estimated_equity, r2.estimated_equity);
        assert_eq!(r1.estimated_ev, r2.estimated_ev);
    }

    #[test]
    fn test_cross_boundary_correctness() {
        // Build the same state both through DTO conversion and natively
        let input = sample_input();
        let wasm_state = to_game_state(&input).unwrap();
        let wasm_config = to_decision_config(input.config.as_ref());

        // Native construction
        use poker_core::card::{Rank, Suit};
        let native_state = GameStateBuilder::new()
            .street(Street::Preflop)
            .hero_cards([
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::Ace, Suit::Hearts),
            ])
            .board(vec![])
            .pot(3)
            .current_bet(2)
            .big_blind(2)
            .min_raise(2)
            .hero_index(0)
            .add_player(Player {
                id: 0,
                position: Position::UTG,
                stack: 998,
                status: PlayerStatus::Active,
                bet_this_round: 0,
            })
            .add_player(Player {
                id: 1,
                position: Position::BTN,
                stack: 998,
                status: PlayerStatus::Active,
                bet_this_round: 0,
            })
            .build()
            .unwrap();

        let native_config = EquityConfig {
            seed: Some(42),
            ..EquityConfig::default()
        };
        let native_decision_config = DecisionConfig {
            equity_config: native_config,
            ..DecisionConfig::default()
        };

        let wasm_engine = poker_decision::decision::DecisionEngine::new(wasm_config);
        let native_engine = poker_decision::decision::DecisionEngine::new(native_decision_config);

        let wasm_result = wasm_engine.analyze(&wasm_state).unwrap();
        let native_result = native_engine.analyze(&native_state).unwrap();

        // Same seed + same input = same result (determinism)
        assert!(
            (wasm_result.estimated_equity - native_result.estimated_equity).abs() < 1e-10,
            "Equity mismatch: WASM={}, native={}",
            wasm_result.estimated_equity,
            native_result.estimated_equity
        );
        assert!(
            (wasm_result.estimated_ev - native_result.estimated_ev).abs() < 1e-10,
            "EV mismatch: WASM={}, native={}",
            wasm_result.estimated_ev,
            native_result.estimated_ev
        );
    }
}
