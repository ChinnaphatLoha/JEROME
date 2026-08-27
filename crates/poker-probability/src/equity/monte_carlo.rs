use crate::equity::{EquityConfig, EquityResult};
use poker_analysis::hand::evaluator::evaluate;
use poker_analysis::range::range::Range;
use poker_core::card::{Card, Deck};
use poker_core::error::PokerError;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;

/// Calculates equity using Monte Carlo sampling.
pub fn calculate_mc_equity(
    hero: [Card; 2],
    opp_range: &Range,
    board: &[Card],
    dead: &Deck,
    config: &EquityConfig,
) -> Result<EquityResult, PokerError> {
    let mut rng = match config.seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_entropy(),
    };

    let mut total_win = 0.0;
    let mut total_tie = 0.0;
    let mut total_loss = 0.0;
    let mut total_samples = 0;

    let mut dead_deck = *dead;
    dead_deck.add(hero[0]);
    dead_deck.add(hero[1]);
    for &c in board {
        dead_deck.add(c);
    }

    let mut valid_combos = Vec::new();
    let mut total_weight = 0.0;

    for combo in opp_range.combos() {
        if dead_deck.contains(combo.cards[0]) || dead_deck.contains(combo.cards[1]) {
            continue;
        }
        valid_combos.push(combo);
        total_weight += combo.weight;
    }

    if total_weight == 0.0 {
        return Ok(EquityResult {
            win: 0.0,
            tie: 0.0,
            loss: 0.0,
            equity: 0.0,
            samples: 0,
            method: "monte_carlo",
        });
    }

    for _ in 0..config.mc_samples {
        let mut r = rng.gen::<f64>() * total_weight;
        let mut selected_combo = valid_combos[0];
        for combo in &valid_combos {
            r -= combo.weight;
            if r <= 0.0 {
                selected_combo = combo;
                break;
            }
        }

        let mut available = Deck::full();
        for c in dead_deck.iter() {
            available.remove_card(c);
        }
        available.remove_card(selected_combo.cards[0]);
        available.remove_card(selected_combo.cards[1]);

        let mut available_cards: Vec<Card> = available.iter().collect();
        let needed = 5 - board.len();

        let mut runout = Vec::with_capacity(needed);
        for _ in 0..needed {
            let idx = rng.gen_range(0..available_cards.len());
            let card = available_cards.swap_remove(idx);
            runout.push(card);
        }

        let mut hero_cards = Vec::with_capacity(7);
        hero_cards.extend_from_slice(&hero);
        hero_cards.extend_from_slice(board);
        hero_cards.extend_from_slice(&runout);

        let mut opp_cards = Vec::with_capacity(7);
        opp_cards.extend_from_slice(&selected_combo.cards);
        opp_cards.extend_from_slice(board);
        opp_cards.extend_from_slice(&runout);

        let hero_rank = evaluate(&hero_cards)?;
        let opp_rank = evaluate(&opp_cards)?;

        match hero_rank.cmp(&opp_rank) {
            Ordering::Greater => total_win += 1.0,
            Ordering::Equal => total_tie += 1.0,
            Ordering::Less => total_loss += 1.0,
        }
        total_samples += 1;
    }

    Ok(EquityResult {
        win: total_win / total_samples as f64,
        tie: total_tie / total_samples as f64,
        loss: total_loss / total_samples as f64,
        equity: (total_win + total_tie / 2.0) / total_samples as f64,
        samples: total_samples,
        method: "monte_carlo",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::card::{Rank, Suit};

    #[test]
    fn test_mc_equity() {
        let hero = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ];
        let opp_range = Range::full();
        let board = vec![];
        let dead = Deck::empty();
        let config = EquityConfig {
            mc_samples: 1000,
            seed: Some(42),
            ..Default::default()
        };

        let result = calculate_mc_equity(hero, &opp_range, &board, &dead, &config).unwrap();
        // AA should have around 85% equity against random hand
        assert!(result.equity > 0.80 && result.equity < 0.90);
    }
}
