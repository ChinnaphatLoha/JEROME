use poker_core::card::{Card, Rank, Suit};
use once_cell::sync::Lazy;

pub const PRIMES: [u64; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];

pub struct LookupTables {
    pub flush: Vec<u32>,
    pub non_flush_5: Vec<(u64, u32)>,
    pub non_flush_6: Vec<(u64, u32)>,
    pub non_flush_7: Vec<(u64, u32)>,
}

pub static TABLES: Lazy<LookupTables> = Lazy::new(|| {
    let mut flush = vec![0; 8192];

    for mask in 0..8192u16 {
        if mask.count_ones() >= 5 {
            let mut cards = Vec::new();
            for r in 0..13 {
                if (mask & (1 << r)) != 0 {
                    cards.push(Card::new(Rank::from_index(r).unwrap(), Suit::Spades));
                }
            }
            if let Ok(hr) = crate::hand::evaluator::evaluate_naive(&cards) {
                flush[mask as usize] = hr.value();
            }
        }
    }

    fn generate_nf(target_count: usize) -> Vec<(u64, u32)> {
        let mut nf = Vec::new();
        let mut current = Vec::new();

        fn backtrack(current: &mut Vec<u8>, start_rank: u8, count: usize, target: usize, nf: &mut Vec<(u64, u32)>) {
            if count == target {
                let mut counts = [0u8; 13];
                for &r in current.iter() {
                    counts[r as usize] += 1;
                }
                if counts.iter().any(|&c| c > 4) { return; }

                let mut prime_prod = 1u64;
                let mut cards = Vec::new();
                let mut suit_idx = 0;
                for (r, &c) in counts.iter().enumerate() {
                    for _ in 0..c {
                        prime_prod *= PRIMES[r];
                        cards.push(Card::new(Rank::from_index(r as u8).unwrap(), Suit::from_index(suit_idx).unwrap()));
                        suit_idx = (suit_idx + 1) % 4;
                    }
                }
                
                if let Ok(hr) = crate::hand::evaluator::evaluate_naive(&cards) {
                    nf.push((prime_prod, hr.value()));
                }
                return;
            }

            for r in start_rank..13 {
                current.push(r);
                backtrack(current, r, count + 1, target, nf);
                current.pop();
            }
        }

        backtrack(&mut current, 0, 0, target_count, &mut nf);
        nf.sort_unstable_by_key(|&(k, _)| k);
        nf
    }

    LookupTables {
        flush,
        non_flush_5: generate_nf(5),
        non_flush_6: generate_nf(6),
        non_flush_7: generate_nf(7),
    }
});

#[inline(always)]
pub fn eval_non_flush(prime_product: u64, cards_len: usize) -> u32 {
    let nf = match cards_len {
        5 => &TABLES.non_flush_5,
        6 => &TABLES.non_flush_6,
        7 => &TABLES.non_flush_7,
        _ => return 0,
    };
    if let Ok(idx) = nf.binary_search_by_key(&prime_product, |&(k, _)| k) {
        nf[idx].1
    } else {
        0
    }
}
