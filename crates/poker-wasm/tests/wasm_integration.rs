//! Integration tests for the WASM boundary.
//!
//! These tests use `wasm-bindgen-test` to exercise the actual `#[wasm_bindgen]`
//! export through the same serialization path that JavaScript consumers use.

use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

// Needed for wasm-bindgen-test to run in Node.js
wasm_bindgen_test_configure!(run_in_browser);

/// Helper: convert a serde_json::Value into a JsValue via serde-wasm-bindgen.
fn js_input(value: serde_json::Value) -> JsValue {
    serde_wasm_bindgen::to_value(&value).expect("Failed to serialize test input")
}

/// Helper: extract a string field from a JsValue object.
fn get_str(obj: &JsValue, key: &str) -> String {
    let key_js = JsValue::from_str(key);
    let val = js_sys::Reflect::get(obj, &key_js).unwrap();
    val.as_string().unwrap_or_default()
}

/// Helper: extract a f64 field from a JsValue object.
fn get_f64(obj: &JsValue, key: &str) -> f64 {
    let key_js = JsValue::from_str(key);
    let val = js_sys::Reflect::get(obj, &key_js).unwrap();
    val.as_f64().unwrap_or(f64::NAN)
}

/// Helper: extract an object field from a JsValue object.
fn get_obj(obj: &JsValue, key: &str) -> JsValue {
    let key_js = JsValue::from_str(key);
    js_sys::Reflect::get(obj, &key_js).unwrap()
}

fn sample_flop_input() -> serde_json::Value {
    json!({
        "hero_cards": ["As", "Kh"],
        "board": ["Qs", "Th", "5d"],
        "street": "flop",
        "pot": 120,
        "current_bet": 40,
        "big_blind": 2,
        "min_raise": 40,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "BTN", "stack": 980, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BB", "stack": 960, "status": "active", "bet_this_round": 40 }
        ],
        "config": { "mc_samples": 5000, "seed": 42 }
    })
}

fn sample_preflop_input() -> serde_json::Value {
    json!({
        "hero_cards": ["As", "Ah"],
        "board": [],
        "street": "preflop",
        "pot": 3,
        "current_bet": 2,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "UTG", "stack": 998, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BTN", "stack": 998, "status": "active", "bet_this_round": 0 }
        ],
        "config": { "seed": 42 }
    })
}

// ─── Happy Path Tests ───────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_analyze_preflop() {
    let input = js_input(sample_preflop_input());
    let result = jerome_poker_wasm::analyze(input).expect("analyze() should succeed for preflop");

    let equity = get_f64(&result, "estimated_equity");
    assert!(
        (0.0..=1.0).contains(&equity),
        "Equity out of range: {}",
        equity
    );

    let ev = get_f64(&result, "estimated_ev");
    assert!(!ev.is_nan(), "EV should be a valid number");

    let action = get_obj(&result, "recommended_action");
    let action_type = get_str(&action, "type");
    assert!(
        ["Fold", "Check", "Call", "Bet", "Raise", "AllIn"].contains(&action_type.as_str()),
        "Unexpected action type: {}",
        action_type
    );
}

#[wasm_bindgen_test]
fn test_analyze_flop() {
    let input = js_input(sample_flop_input());
    let result = jerome_poker_wasm::analyze(input).expect("analyze() should succeed for flop");

    let equity = get_f64(&result, "estimated_equity");
    assert!((0.0..=1.0).contains(&equity));

    let alternatives = get_obj(&result, "alternatives");
    assert!(
        js_sys::Array::is_array(&alternatives),
        "alternatives should be an array"
    );
    let alts = js_sys::Array::from(&alternatives);
    assert!(alts.length() > 0, "Should have at least one alternative");
}

#[wasm_bindgen_test]
fn test_analyze_turn() {
    let input = js_input(json!({
        "hero_cards": ["As", "Kh"],
        "board": ["Qs", "Th", "5d", "2c"],
        "street": "turn",
        "pot": 200,
        "current_bet": 60,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "BTN", "stack": 920, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BB", "stack": 900, "status": "active", "bet_this_round": 60 }
        ],
        "config": { "seed": 42 }
    }));
    let result = jerome_poker_wasm::analyze(input).expect("analyze() should succeed for turn");
    let equity = get_f64(&result, "estimated_equity");
    assert!((0.0..=1.0).contains(&equity));
}

#[wasm_bindgen_test]
fn test_analyze_river() {
    let input = js_input(json!({
        "hero_cards": ["As", "Kh"],
        "board": ["Qs", "Th", "5d", "2c", "9h"],
        "street": "river",
        "pot": 400,
        "current_bet": 0,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "BTN", "stack": 800, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BB", "stack": 800, "status": "active", "bet_this_round": 0 }
        ],
        "config": { "seed": 42 }
    }));
    let result = jerome_poker_wasm::analyze(input).expect("analyze() should succeed for river");
    let equity = get_f64(&result, "estimated_equity");
    assert!((0.0..=1.0).contains(&equity));
}

// ─── Determinism ────────────────────────────────────────────────────

#[wasm_bindgen_test]
fn test_deterministic_across_calls() {
    let input1 = js_input(sample_flop_input());
    let input2 = js_input(sample_flop_input());

    let result1 = jerome_poker_wasm::analyze(input1).unwrap();
    let result2 = jerome_poker_wasm::analyze(input2).unwrap();

    let eq1 = get_f64(&result1, "estimated_equity");
    let eq2 = get_f64(&result2, "estimated_equity");
    assert_eq!(
        eq1, eq2,
        "Same input + same seed should produce same equity"
    );

    let ev1 = get_f64(&result1, "estimated_ev");
    let ev2 = get_f64(&result2, "estimated_ev");
    assert_eq!(ev1, ev2, "Same input + same seed should produce same EV");
}

// ─── Validation Error Tests ─────────────────────────────────────────

#[wasm_bindgen_test]
fn test_error_invalid_card() {
    let input = js_input(json!({
        "hero_cards": ["Xx", "Ah"],
        "board": [],
        "street": "preflop",
        "pot": 3,
        "current_bet": 2,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "UTG", "stack": 998, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BTN", "stack": 998, "status": "active", "bet_this_round": 0 }
        ]
    }));
    let result = jerome_poker_wasm::analyze(input);
    assert!(result.is_err(), "Should fail for invalid card");

    let err = result.unwrap_err();
    // Error should be a js_sys::Error with a code property
    let code = get_str(&err, "code");
    assert_eq!(code, "INVALID_CARD", "Error code should be INVALID_CARD");
}

#[wasm_bindgen_test]
fn test_error_invalid_street() {
    let input = js_input(json!({
        "hero_cards": ["As", "Ah"],
        "board": [],
        "street": "postflop",
        "pot": 3,
        "current_bet": 2,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "UTG", "stack": 998, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BTN", "stack": 998, "status": "active", "bet_this_round": 0 }
        ]
    }));
    let result = jerome_poker_wasm::analyze(input);
    assert!(result.is_err());
    let code = get_str(&result.unwrap_err(), "code");
    assert_eq!(code, "INVALID_ENUM");
}

#[wasm_bindgen_test]
fn test_error_duplicate_cards() {
    let input = js_input(json!({
        "hero_cards": ["As", "As"],
        "board": [],
        "street": "preflop",
        "pot": 3,
        "current_bet": 2,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "UTG", "stack": 998, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BTN", "stack": 998, "status": "active", "bet_this_round": 0 }
        ]
    }));
    let result = jerome_poker_wasm::analyze(input);
    assert!(result.is_err());
    let code = get_str(&result.unwrap_err(), "code");
    assert_eq!(code, "ENGINE_ERROR");
}

#[wasm_bindgen_test]
fn test_error_wrong_board_count() {
    let input = js_input(json!({
        "hero_cards": ["As", "Kh"],
        "board": ["Qs"],
        "street": "flop",
        "pot": 100,
        "current_bet": 0,
        "big_blind": 2,
        "hero_index": 0,
        "players": [
            { "id": 0, "position": "BTN", "stack": 980, "status": "active", "bet_this_round": 0 },
            { "id": 1, "position": "BB", "stack": 980, "status": "active", "bet_this_round": 0 }
        ]
    }));
    let result = jerome_poker_wasm::analyze(input);
    assert!(result.is_err());
    let code = get_str(&result.unwrap_err(), "code");
    assert_eq!(code, "ENGINE_ERROR");
}

#[wasm_bindgen_test]
fn test_error_malformed_input() {
    // Completely wrong shape
    let input = js_input(json!({ "foo": "bar" }));
    let result = jerome_poker_wasm::analyze(input);
    assert!(result.is_err());
    let code = get_str(&result.unwrap_err(), "code");
    assert_eq!(code, "DESERIALIZATION_ERROR");
}

// ─── Output Structure Tests ─────────────────────────────────────────

#[wasm_bindgen_test]
fn test_output_has_all_expected_fields() {
    let input = js_input(sample_preflop_input());
    let result = jerome_poker_wasm::analyze(input).unwrap();

    // Top-level fields exist and have correct types
    assert!(!get_obj(&result, "recommended_action").is_undefined());
    assert!(!get_f64(&result, "estimated_equity").is_nan());
    assert!(!get_f64(&result, "required_equity").is_nan());
    assert!(!get_f64(&result, "estimated_ev").is_nan());
    assert!(js_sys::Array::is_array(&get_obj(&result, "alternatives")));
    assert!(!get_obj(&result, "explanation").is_undefined());

    // Action structure
    let action = get_obj(&result, "recommended_action");
    let action_type = get_str(&action, "type");
    assert!(!action_type.is_empty());

    // Explanation structure
    let explanation = get_obj(&result, "explanation");
    assert!(!get_str(&explanation, "recommended_action_label").is_empty());
    assert!(js_sys::Array::is_array(&get_obj(&explanation, "factors")));
}
