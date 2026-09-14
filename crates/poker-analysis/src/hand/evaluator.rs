use super::hand_rank::{HandCategory, HandRank};
use poker_core::card::Card;
use poker_core::error::PokerError;
use crate::hand::lookup::{PRIMES, TABLES};
use std::cmp::max;

/// Evaluates a collection of 5 to 7 cards and returns the best 5-card poker hand using lookup tables.
pub fn evaluate(cards: &[Card]) -> Result<HandRank, PokerError> {
    let n = cards.len();
    if n < 5 || n > 7 {
        return Err(PokerError::InsufficientCards {
            needed: 5,
            available: n,
        });
    }

    let mut suit_masks = [0u16; 4];
    let mut prime_prod = 1;

    for card in cards {
        suit_masks[card.suit().index() as usize] |= 1 << card.rank().index();
        prime_prod *= PRIMES[card.rank().index() as usize];
    }

    let mut flush_val = 0;
    for &mask in &suit_masks {
        if mask.count_ones() >= 5 {
            flush_val = TABLES.flush[mask as usize];
            break;
        }
    }

    let max_val = crate::hand::lookup::eval_non_flush(prime_prod, n);

    Ok(HandRank::from_value(max(flush_val, max_val)))
}

/// The naive evaluator used to initialize the lookup tables.
pub fn evaluate_naive(cards: &[Card]) -> Result<HandRank, PokerError> {
    if cards.len() < 5 {
        return Err(PokerError::InsufficientCards {
            needed: 5,
            available: cards.len(),
        });
    }

    let mut ranks = [0u8; 13];
    let mut suits = [0u8; 4];
    for card in cards {
        ranks[card.rank().index() as usize] += 1;
        suits[card.suit().index() as usize] += 1;
    }

    let mut flush_suit = None;
    for (s, &count) in suits.iter().enumerate() {
        if count >= 5 {
            flush_suit = Some(s as u8);
            break;
        }
    }

    let mut straight_high = None;
    let mut consecutive = 0;
    for r in (0..13).rev() {
        if ranks[r] > 0 {
            consecutive += 1;
            if consecutive >= 5 {
                straight_high = Some((r + 4) as u8);
                break;
            }
        } else {
            consecutive = 0;
        }
    }

    if straight_high.is_none()
        && ranks[12] > 0
        && ranks[0] > 0
        && ranks[1] > 0
        && ranks[2] > 0
        && ranks[3] > 0
    {
        straight_high = Some(3); // 5 high straight
    }

    if let Some(fs) = flush_suit {
        let mut flush_cards = Vec::new();
        for card in cards {
            if card.suit().index() == fs {
                flush_cards.push(card.rank().index());
            }
        }
        flush_cards.sort_unstable_by(|a, b| b.cmp(a));
        flush_cards.dedup();

        let mut sf_high = None;
        let mut cons = 0;
        let mut last = 255;
        for &r in &flush_cards {
            if last == 255 || last == r + 1 {
                cons += 1;
                if cons >= 5 {
                    sf_high = Some(r + 4);
                    break;
                }
            } else {
                cons = 1;
            }
            last = r;
        }
        if sf_high.is_none()
            && flush_cards.contains(&12)
            && flush_cards.contains(&0)
            && flush_cards.contains(&1)
            && flush_cards.contains(&2)
            && flush_cards.contains(&3)
        {
            sf_high = Some(3);
        }

        if let Some(h) = sf_high {
            return Ok(HandRank::new(HandCategory::StraightFlush, &[h, 0, 0, 0, 0]));
        }
    }

    for r in (0..13).rev() {
        if ranks[r] == 4 {
            let mut kicker = 0;
            for k in (0..13).rev() {
                if k != r && ranks[k] > 0 {
                    kicker = k as u8;
                    break;
                }
            }
            return Ok(HandRank::new(
                HandCategory::FourOfAKind,
                &[r as u8, kicker, 0, 0, 0],
            ));
        }
    }

    let mut trips_rank = None;
    let mut pair_rank = None;
    for r in (0..13).rev() {
        if ranks[r] == 3 {
            if trips_rank.is_none() {
                trips_rank = Some(r as u8);
            } else if pair_rank.is_none() {
                pair_rank = Some(r as u8);
            }
        } else if ranks[r] >= 2 {
            if pair_rank.is_none() {
                pair_rank = Some(r as u8);
            }
        }
    }
    if let (Some(t), Some(p)) = (trips_rank, pair_rank) {
        return Ok(HandRank::new(HandCategory::FullHouse, &[t, p, 0, 0, 0]));
    }

    if let Some(fs) = flush_suit {
        let mut flush_cards = Vec::new();
        for card in cards {
            if card.suit().index() == fs {
                flush_cards.push(card.rank().index());
            }
        }
        flush_cards.sort_unstable_by(|a, b| b.cmp(a));
        return Ok(HandRank::new(HandCategory::Flush, &flush_cards[0..5]));
    }

    if let Some(h) = straight_high {
        return Ok(HandRank::new(HandCategory::Straight, &[h, 0, 0, 0, 0]));
    }

    if let Some(t) = trips_rank {
        let mut kickers = vec![t];
        for r in (0..13).rev() {
            if r != t as usize && ranks[r] > 0 {
                kickers.push(r as u8);
                if kickers.len() == 3 {
                    break;
                }
            }
        }
        return Ok(HandRank::new(HandCategory::ThreeOfAKind, &kickers));
    }

    let mut pairs = Vec::new();
    for r in (0..13).rev() {
        if ranks[r] == 2 {
            pairs.push(r as u8);
        }
    }

    if pairs.len() >= 2 {
        let p1 = pairs[0];
        let p2 = pairs[1];
        let mut kicker = 0;
        for r in (0..13).rev() {
            if r != p1 as usize && r != p2 as usize && ranks[r] > 0 {
                kicker = r as u8;
                break;
            }
        }
        return Ok(HandRank::new(
            HandCategory::TwoPair,
            &[p1, p2, kicker, 0, 0],
        ));
    }

    if pairs.len() == 1 {
        let p1 = pairs[0];
        let mut kickers = vec![p1];
        for r in (0..13).rev() {
            if r != p1 as usize && ranks[r] > 0 {
                kickers.push(r as u8);
                if kickers.len() == 4 {
                    break;
                }
            }
        }
        return Ok(HandRank::new(HandCategory::Pair, &kickers));
    }

    let mut kickers = Vec::new();
    for r in (0..13).rev() {
        if ranks[r] > 0 {
            kickers.push(r as u8);
            if kickers.len() == 5 {
                break;
            }
        }
    }
    Ok(HandRank::new(HandCategory::HighCard, &kickers))
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_core::card::Card;

    fn parse_cards(s: &str) -> Vec<Card> {
        s.split_whitespace()
            .map(|c| Card::from_str(c).unwrap())
            .collect()
    }

    #[test]
    fn test_royal_flush() {
        let cards = parse_cards("Ah Kh Qh Jh Th 2s 3d");
        let rank = evaluate(&cards).unwrap();
        assert_eq!(rank.category(), HandCategory::StraightFlush);
        assert_eq!(rank.kickers()[0], 12); // Ace high
    }

    #[test]
    fn test_wheel_straight() {
        let cards = parse_cards("Ah 2s 3d 4c 5h 9s Td");
        let rank = evaluate(&cards).unwrap();
        assert_eq!(rank.category(), HandCategory::Straight);
        assert_eq!(rank.kickers()[0], 3); // 5 high
    }

    #[test]
    fn test_two_pair() {
        let cards = parse_cards("Ah As Kh Ks 2d");
        let rank = evaluate(&cards).unwrap();
        assert_eq!(rank.category(), HandCategory::TwoPair);
        assert_eq!(rank.kickers()[0], 12);
        assert_eq!(rank.kickers()[1], 11);
        assert_eq!(rank.kickers()[2], 0);
    }
}
