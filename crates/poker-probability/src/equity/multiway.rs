use crate::equity::{EquityConfig, EquityResult};
use poker_analysis::hand::evaluator::evaluate;
use poker_analysis::range::range::Range;
use poker_core::card::{Card, Deck};
use poker_core::error::PokerError;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;

/// Per-player equity result in a multi-way pot.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiWayEquityResult {
    /// Equity for each player (index 0 = hero).
    pub equities: Vec<f64>,
    /// Win probability for each player.
    pub win_rates: Vec<f64>,
    /// Tie probability for each player.
    pub tie_rates: Vec<f64>,
    /// Total number of Monte Carlo samples evaluated.
    pub samples: usize,
    /// Number of players in the pot.
    pub num_players: usize,
}

/// Calculates multi-way pot equity using Monte Carlo sampling.
///
/// `hero` — Hero's hole cards.
/// `opponent_ranges` — One `Range` per opponent (1..N opponents).
/// `board` — Community cards dealt so far (0–5).
/// `dead` — Known dead/mucked cards.
/// `config` — MC sample count, optional seed.
///
/// Returns `MultiWayEquityResult` with per-player equities where
/// index 0 is the hero and indices 1..N are opponents in order.
pub fn calculate_multiway_equity(
    hero: [Card; 2],
    opponent_ranges: &[Range],
    board: &[Card],
    dead: &Deck,
    config: &EquityConfig,
) -> Result<MultiWayEquityResult, PokerError> {
    let num_opponents = opponent_ranges.len();
    let num_players = num_opponents + 1; // hero + opponents

    if num_opponents == 0 {
        return Ok(MultiWayEquityResult {
            equities: vec![1.0],
            win_rates: vec![1.0],
            tie_rates: vec![0.0],
            samples: 0,
            num_players: 1,
        });
    }

    let mut rng = match config.seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_entropy(),
    };

    // Build dead-card mask.
    let mut dead_deck = *dead;
    dead_deck.add(hero[0]);
    dead_deck.add(hero[1]);
    for &c in board {
        dead_deck.add(c);
    }

    // Pre-filter valid combos per opponent.
    let mut valid_combos_per_opp: Vec<Vec<&poker_analysis::range::combo::Combo>> = Vec::with_capacity(num_opponents);
    let mut weights_per_opp: Vec<f64> = Vec::with_capacity(num_opponents);

    for opp_range in opponent_ranges {
        let mut valid = Vec::new();
        let mut total_w = 0.0;
        for combo in opp_range.combos() {
            if !dead_deck.contains(combo.cards[0]) && !dead_deck.contains(combo.cards[1]) {
                total_w += combo.weight;
                valid.push(combo);
            }
        }
        if valid.is_empty() {
            return Ok(MultiWayEquityResult {
                equities: vec![0.0; num_players],
                win_rates: vec![0.0; num_players],
                tie_rates: vec![0.0; num_players],
                samples: 0,
                num_players,
            });
        }
        valid_combos_per_opp.push(valid);
        weights_per_opp.push(total_w);
    }

    let mut total_wins = vec![0.0_f64; num_players];
    let mut total_ties = vec![0.0_f64; num_players];
    let needed = 5 - board.len();

    for _ in 0..config.mc_samples {
        // Track cards used in this sample.
        let mut sample_dead = dead_deck;

        // Sample one combo per opponent, ensuring no card collisions.
        let mut opp_hands: Vec<[Card; 2]> = Vec::with_capacity(num_opponents);
        let mut valid_sample = true;

        for opp_idx in 0..num_opponents {
            let total_w = weights_per_opp[opp_idx];
            let combos = &valid_combos_per_opp[opp_idx];

            // Rejection sampling: try to find a combo that doesn't collide.
            let mut found = false;
            for _ in 0..50 {
                let mut r = rng.gen::<f64>() * total_w;
                let mut selected = combos[0];
                for combo in combos.iter() {
                    r -= combo.weight;
                    if r <= 0.0 {
                        selected = combo;
                        break;
                    }
                }

                if !sample_dead.contains(selected.cards[0])
                    && !sample_dead.contains(selected.cards[1])
                {
                    sample_dead.add(selected.cards[0]);
                    sample_dead.add(selected.cards[1]);
                    opp_hands.push(selected.cards);
                    found = true;
                    break;
                }
            }

            if !found {
                valid_sample = false;
                break;
            }
        }

        if !valid_sample {
            continue;
        }

        // Sample runout cards.
        let mut available = Deck::full();
        for c in sample_dead.iter() {
            available.remove_card(c);
        }
        let mut available_cards: Vec<Card> = available.iter().collect();

        if available_cards.len() < needed {
            continue;
        }

        let mut runout = Vec::with_capacity(needed);
        for _ in 0..needed {
            let idx = rng.gen_range(0..available_cards.len());
            let card = available_cards.swap_remove(idx);
            runout.push(card);
        }

        // Evaluate all hands.
        let mut hero_cards = Vec::with_capacity(7);
        hero_cards.extend_from_slice(&hero);
        hero_cards.extend_from_slice(board);
        hero_cards.extend_from_slice(&runout);
        let hero_rank = evaluate(&hero_cards)?;

        let mut all_ranks = Vec::with_capacity(num_players);
        all_ranks.push(hero_rank);

        for opp_hand in &opp_hands {
            let mut opp_cards = Vec::with_capacity(7);
            opp_cards.extend_from_slice(opp_hand);
            opp_cards.extend_from_slice(board);
            opp_cards.extend_from_slice(&runout);
            all_ranks.push(evaluate(&opp_cards)?);
        }

        // Find the best hand(s).
        let best_rank = *all_ranks.iter().max().unwrap();
        let winners: Vec<usize> = all_ranks
            .iter()
            .enumerate()
            .filter(|(_, r)| **r == best_rank)
            .map(|(i, _)| i)
            .collect();

        if winners.len() == 1 {
            total_wins[winners[0]] += 1.0;
        } else {
            let share = 1.0 / winners.len() as f64;
            for &w in &winners {
                total_ties[w] += share;
            }
        }
    }

    let total_samples = config.mc_samples;
    let total_f = total_samples as f64;

    let equities: Vec<f64> = (0..num_players)
        .map(|i| (total_wins[i] + total_ties[i]) / total_f)
        .collect();

    let win_rates: Vec<f64> = total_wins.iter().map(|w| w / total_f).collect();
    let tie_rates: Vec<f64> = total_ties.iter().map(|t| t / total_f).collect();

    Ok(MultiWayEquityResult {
        equities,
        win_rates,
        tie_rates,
        samples: total_samples,
        num_players,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_analysis::range::combo::Combo;
    use poker_core::card::{Rank, Suit};

    #[test]
    fn test_multiway_3_players() {
        // Hero: AA
        let hero = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ];

        // Opponent 1: KK
        let mut opp1_range = Range::empty();
        opp1_range.add_combo(Combo::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            1.0,
        ));

        // Opponent 2: QQ
        let mut opp2_range = Range::empty();
        opp2_range.add_combo(Combo::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            1.0,
        ));

        let board = vec![];
        let dead = Deck::empty();
        let config = EquityConfig {
            mc_samples: 5000,
            seed: Some(42),
            ..Default::default()
        };

        let result =
            calculate_multiway_equity(hero, &[opp1_range, opp2_range], &board, &dead, &config)
                .unwrap();

        assert_eq!(result.num_players, 3);
        assert_eq!(result.equities.len(), 3);

        // AA should dominate in a 3-way pot against KK and QQ.
        assert!(result.equities[0] > 0.60, "Hero equity: {}", result.equities[0]);
        // All equities should sum to approximately 1.0.
        let sum: f64 = result.equities.iter().sum();
        assert!(
            (sum - 1.0).abs() < 0.05,
            "Equity sum should be ~1.0, got: {}",
            sum
        );
    }

    #[test]
    fn test_multiway_heads_up_consistency() {
        // With one opponent, multi-way should approximate heads-up equity.
        let hero = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ];

        let opp_range = Range::full();
        let board = vec![];
        let dead = Deck::empty();
        let config = EquityConfig {
            mc_samples: 5000,
            seed: Some(42),
            ..Default::default()
        };

        let result =
            calculate_multiway_equity(hero, &[opp_range], &board, &dead, &config).unwrap();

        assert_eq!(result.num_players, 2);
        // AA vs random should be ~85%.
        assert!(
            result.equities[0] > 0.80 && result.equities[0] < 0.92,
            "Hero equity: {}",
            result.equities[0]
        );
    }

    #[test]
    fn test_multiway_4_players() {
        let hero = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ];

        let mut opp1 = Range::empty();
        opp1.add_combo(Combo::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            1.0,
        ));
        let mut opp2 = Range::empty();
        opp2.add_combo(Combo::new(
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            1.0,
        ));
        let mut opp3 = Range::empty();
        opp3.add_combo(Combo::new(
            Card::new(Rank::Jack, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
            1.0,
        ));

        let board = vec![];
        let dead = Deck::empty();
        let config = EquityConfig {
            mc_samples: 5000,
            seed: Some(42),
            ..Default::default()
        };

        let result =
            calculate_multiway_equity(hero, &[opp1, opp2, opp3], &board, &dead, &config)
                .unwrap();

        assert_eq!(result.num_players, 4);
        // AA should still be the favorite.
        assert!(result.equities[0] > 0.50, "Hero equity: {}", result.equities[0]);
        let sum: f64 = result.equities.iter().sum();
        assert!(
            (sum - 1.0).abs() < 0.05,
            "Equity sum should be ~1.0, got: {}",
            sum
        );
    }
}
