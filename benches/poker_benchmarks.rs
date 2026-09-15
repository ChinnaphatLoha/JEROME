use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poker_core::card::{Card, Rank, Suit};
use poker_core::game::GameStateBuilder;
use poker_core::Position;
use poker_engine::{EngineConfig, PokerEngine};
use poker_core::game::{Player, PlayerStatus};

fn create_valid_player(id: u8, position: Position) -> Player {
    Player {
        id,
        position,
        stack: 1000,
        status: PlayerStatus::Active,
        bet_this_round: 0,
    }
}

fn bench_hand_evaluation(c: &mut Criterion) {
    let cards = [
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::King, Suit::Spades),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Ten, Suit::Spades),
        Card::new(Rank::Two, Suit::Hearts),
        Card::new(Rank::Three, Suit::Diamonds),
    ];
    c.bench_function("hand_evaluation", |b| {
        b.iter(|| {
            black_box(poker_analysis::hand::evaluator::evaluate(black_box(&cards)).unwrap());
        })
    });
}

fn bench_equity_calculation(c: &mut Criterion) {
    c.bench_function("equity_calculation", |b| {
        b.iter(|| {
            // Dummy for equity MC
            black_box(1);
        })
    });
}

fn bench_full_decision_pipeline(c: &mut Criterion) {
    let state = GameStateBuilder::new()
        .hero_cards([
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ])
        .add_player(create_valid_player(0, Position::UTG))
        .add_player(create_valid_player(1, Position::BTN))
        .hero_index(0)
        .build()
        .unwrap();

    let engine = PokerEngine::new(EngineConfig::default());

    c.bench_function("full_decision_pipeline", |b| {
        b.iter(|| {
            let _ = engine.analyze(black_box(&state));
        })
    });
}

criterion_group!(
    benches,
    bench_hand_evaluation,
    bench_equity_calculation,
    bench_full_decision_pipeline
);
criterion_main!(benches);
