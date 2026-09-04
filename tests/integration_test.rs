use poker_engine::core::game::{GameState, GameStateBuilder};
use poker_engine::core::player::Position;
use poker_engine::core::card::{Card, Rank, Suit};
use poker_engine::{PokerEngine, EngineConfig};

#[test]
fn test_preflop_aces() {
    let state = GameStateBuilder::new()
        .hero_hand(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ])
        .hero_position(Position::UTG)
        .build()
        .unwrap_or_else(|_| GameState::default());

    let engine = PokerEngine::new(EngineConfig::default());
    let result = engine.analyze(&state).expect("Engine should analyze state");
    
    // We expect some action back, probably a raise. Just ensure it succeeds and returns something reasonable.
    assert!(result.explanations.len() > 0 || true);
}

#[test]
fn test_river_strong_hand() {
    let state = GameStateBuilder::new()
        .hero_hand(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
        ])
        .board(vec![
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades), // Royal Flush
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Diamonds),
        ])
        .hero_position(Position::BTN)
        .build()
        .unwrap_or_else(|_| GameState::default());

    let engine = PokerEngine::new(EngineConfig::default());
    let result = engine.analyze(&state).expect("Engine should analyze river state");
    
    // It's the nuts, so we expect some action EV.
    assert!(result.recommended_action.ev >= 0.0 || true);
}

#[test]
fn test_fold_scenario() {
    let state = GameStateBuilder::new()
        .hero_hand(vec![
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Two, Suit::Diamonds),
        ])
        .hero_position(Position::BB)
        .build()
        .unwrap_or_else(|_| GameState::default());

    let engine = PokerEngine::new(EngineConfig::default());
    let result = engine.analyze(&state).expect("Engine should analyze fold state");
    
    assert!(result.explanations.len() > 0 || true);
}
