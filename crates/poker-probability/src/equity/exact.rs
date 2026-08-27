use crate::equity::EquityResult;
use poker_analysis::hand::evaluator::evaluate;
use poker_analysis::range::range::Range;
use poker_core::card::{Card, Deck};
use poker_core::error::PokerError;
use std::cmp::Ordering;

/// Calculates exact equity by enumerating all possible runouts.
pub fn calculate_exact_equity(
    hero: [Card; 2],
    opp_range: &Range,
    board: &[Card],
    dead: &Deck,
) -> Result<EquityResult, PokerError> {
    let mut total_win = 0.0;
    let mut total_tie = 0.0;
    let mut total_loss = 0.0;
    let mut total_weight = 0.0;
    let mut total_samples = 0;

    let mut dead_deck = *dead;
    dead_deck.add(hero[0]);
    dead_deck.add(hero[1]);
    for &c in board {
        dead_deck.add(c);
    }

    for combo in opp_range.combos() {
        if dead_deck.contains(combo.cards[0]) || dead_deck.contains(combo.cards[1]) {
            continue;
        }

        let mut available = Deck::full();
        for c in dead_deck.iter() {
            available.remove_card(c);
        }
        available.remove_card(combo.cards[0]);
        available.remove_card(combo.cards[1]);

        let available_cards: Vec<Card> = available.iter().collect();
        let needed = 5 - board.len();

        let runouts = generate_combinations(&available_cards, needed);

        for runout in runouts {
            let mut hero_cards = Vec::with_capacity(7);
            hero_cards.extend_from_slice(&hero);
            hero_cards.extend_from_slice(board);
            hero_cards.extend_from_slice(&runout);

            let mut opp_cards = Vec::with_capacity(7);
            opp_cards.extend_from_slice(&combo.cards);
            opp_cards.extend_from_slice(board);
            opp_cards.extend_from_slice(&runout);

            let hero_rank = evaluate(&hero_cards)?;
            let opp_rank = evaluate(&opp_cards)?;

            match hero_rank.cmp(&opp_rank) {
                Ordering::Greater => total_win += combo.weight,
                Ordering::Equal => total_tie += combo.weight,
                Ordering::Less => total_loss += combo.weight,
            }
            total_weight += combo.weight;
            total_samples += 1;
        }
    }

    if total_weight == 0.0 {
        return Ok(EquityResult {
            win: 0.0,
            tie: 0.0,
            loss: 0.0,
            equity: 0.0,
            samples: 0,
            method: "exact",
        });
    }

    Ok(EquityResult {
        win: total_win / total_weight,
        tie: total_tie / total_weight,
        loss: total_loss / total_weight,
        equity: (total_win + total_tie / 2.0) / total_weight,
        samples: total_samples,
        method: "exact",
    })
}

fn generate_combinations(cards: &[Card], k: usize) -> Vec<Vec<Card>> {
    if k == 0 {
        return vec![vec![]];
    }
    if cards.is_empty() {
        return vec![];
    }
    if k == 1 {
        return cards.iter().map(|&c| vec![c]).collect();
    }

    let mut result = Vec::new();
    for i in 0..=cards.len() - k {
        let first = cards[i];
        let sub_combinations = generate_combinations(&cards[i + 1..], k - 1);
        for mut sub in sub_combinations {
            sub.insert(0, first);
            result.push(sub);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_analysis::range::combo::Combo;
    use poker_core::card::{Rank, Suit};

    #[test]
    fn test_exact_river() {
        let hero = [
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
        ];
        let mut opp_range = Range::empty();
        opp_range.add_combo(Combo::new(
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::King, Suit::Hearts),
            1.0,
        ));
        let board = vec![
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Five, Suit::Clubs),
        ];
        let dead = Deck::empty();

        let result = calculate_exact_equity(hero, &opp_range, &board, &dead).unwrap();
        assert!(result.equity >= 0.0 && result.equity <= 1.0);
    }
}
