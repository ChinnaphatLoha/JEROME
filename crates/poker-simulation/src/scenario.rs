use poker_core::card::{Card, Rank, Suit};
use poker_core::game::PlayerStatus;
use poker_core::game::{GameState, GameStateBuilder, Player};
use poker_core::ActionType;
use poker_core::Position;

/// Represents a specific poker scenario for testing and simulation.
pub struct Scenario {
    pub name: String,
    pub game_state: GameState,
    pub expected_action: Option<ActionType>,
    pub description: String,
}

fn create_valid_player(id: u8, position: Position) -> Player {
    Player {
        id,
        position,
        stack: 1000,
        status: PlayerStatus::Active,
        bet_this_round: 0,
    }
}

/// Creates a set of common preflop scenarios.
pub fn preflop_scenarios() -> Vec<Scenario> {
    vec![
        Scenario {
            name: "Hero UTG with AA".to_string(),
            game_state: GameStateBuilder::new()
                .hero_cards([
                    Card::new(Rank::Ace, Suit::Spades),
                    Card::new(Rank::Ace, Suit::Hearts),
                ])
                .add_player(create_valid_player(0, Position::UTG))
                .add_player(create_valid_player(1, Position::BTN))
                .hero_index(0)
                .build()
                .unwrap(),
            expected_action: Some(ActionType::Raise(3)),
            description: "Premium hand in early position, expected to raise.".to_string(),
        },
        Scenario {
            name: "Hero BTN with 72o".to_string(),
            game_state: GameStateBuilder::new()
                .hero_cards([
                    Card::new(Rank::Seven, Suit::Clubs),
                    Card::new(Rank::Two, Suit::Diamonds),
                ])
                .add_player(create_valid_player(0, Position::UTG))
                .add_player(create_valid_player(1, Position::BTN))
                .hero_index(1)
                .build()
                .unwrap(),
            expected_action: Some(ActionType::Fold),
            description: "Weak hand, should be folded.".to_string(),
        },
    ]
}

/// Creates a set of common postflop scenarios.
pub fn postflop_scenarios() -> Vec<Scenario> {
    vec![Scenario {
        name: "Hero top pair dry board".to_string(),
        game_state: GameStateBuilder::new()
            .hero_cards([
                Card::new(Rank::Ace, Suit::Spades),
                Card::new(Rank::King, Suit::Hearts),
            ])
            .board(vec![
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Diamonds),
                Card::new(Rank::Two, Suit::Spades),
            ])
            .add_player(create_valid_player(0, Position::BTN))
            .add_player(create_valid_player(1, Position::BB))
            .hero_index(0)
            .street(poker_core::game::Street::Flop)
            .build()
            .unwrap(),
        expected_action: Some(ActionType::Bet(5)),
        description: "Top pair top kicker on a dry board, expected to bet.".to_string(),
    }]
}
