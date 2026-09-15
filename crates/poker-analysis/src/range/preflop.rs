use super::builder::RangeBuilder;
use super::range::Range;
use poker_core::card::Rank;
use poker_core::game::position::Position;

/// Provides GTO-approximate preflop opening ranges (Raise First In - RFI)
/// for different positions in a 6-max or 9-max No Limit Texas Hold'em game.
///
/// These ranges are simplified for the purpose of the decision engine.
pub struct PreflopRanges;

impl PreflopRanges {
    /// Returns the Raise First In (RFI) range for a given position.
    pub fn rfi_range(position: Position) -> Range {
        match position {
            Position::UTG | Position::UTG1 => Self::utg_rfi(),
            Position::MP | Position::MP1 | Position::HJ => Self::hj_rfi(),
            Position::CO => Self::co_rfi(),
            Position::BTN => Self::btn_rfi(),
            Position::SB => Self::sb_rfi(),
            Position::BB => Self::bb_defense(), // Technically not RFI, but the default range to play
        }
    }

    /// Under The Gun (UTG) opening range (~15% of hands).
    /// Typically: 77+, ATs+, KTs+, QTs+, JTs, AJo+, KQo
    fn utg_rfi() -> Range {
        let mut builder = RangeBuilder::new();
        
        // Pairs: 77+
        for r in 5..=12 {
            builder = builder.add_pair(Rank::from_index(r as u8).unwrap(), 1.0);
        }

        // Suited Ax: ATs+
        for r in 8..=11 {
            builder = builder.add_suited(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Kx: KTs+
        for r in 8..=10 {
            builder = builder.add_suited(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Qx: QTs+
        for r in 8..=9 {
            builder = builder.add_suited(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited JTs
        builder = builder.add_suited(Rank::Jack, Rank::Ten, 1.0);

        // Offsuit Ax: AJo+
        builder = builder.add_offsuit(Rank::Ace, Rank::Jack, 1.0);
        builder = builder.add_offsuit(Rank::Ace, Rank::Queen, 1.0);
        builder = builder.add_offsuit(Rank::Ace, Rank::King, 1.0);
        // Offsuit Kx: KQo
        builder = builder.add_offsuit(Rank::King, Rank::Queen, 1.0);

        builder.build()
    }

    /// Hijack (HJ) opening range (~20% of hands).
    /// UTG + 55+, A8s+, K9s+, Q9s+, J9s+, T9s, ATo+, KJo+
    fn hj_rfi() -> Range {
        let mut builder = RangeBuilder::new();
        
        // Pairs: 55+
        for r in 3..=12 {
            builder = builder.add_pair(Rank::from_index(r as u8).unwrap(), 1.0);
        }

        // Suited Ax: A8s+
        for r in 6..=11 {
            builder = builder.add_suited(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Kx: K9s+
        for r in 7..=10 {
            builder = builder.add_suited(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Qx: Q9s+
        for r in 7..=9 {
            builder = builder.add_suited(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Jx: J9s+
        for r in 7..=8 {
            builder = builder.add_suited(Rank::Jack, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited T9s
        builder = builder.add_suited(Rank::Ten, Rank::Nine, 1.0);

        // Offsuit Ax: ATo+
        builder = builder.add_offsuit(Rank::Ace, Rank::Ten, 1.0);
        builder = builder.add_offsuit(Rank::Ace, Rank::Jack, 1.0);
        builder = builder.add_offsuit(Rank::Ace, Rank::Queen, 1.0);
        builder = builder.add_offsuit(Rank::Ace, Rank::King, 1.0);
        // Offsuit Kx: KJo+
        builder = builder.add_offsuit(Rank::King, Rank::Jack, 1.0);
        builder = builder.add_offsuit(Rank::King, Rank::Queen, 1.0);

        builder.build()
    }

    /// Cutoff (CO) opening range (~28% of hands).
    /// HJ + 22+, A2s+, K6s+, Q8s+, J8s+, T8s+, 98s, 87s, A9o+, KTo+, QJo
    fn co_rfi() -> Range {
        let mut builder = RangeBuilder::new();
        
        // Pairs: 22+
        for r in 0..=12 {
            builder = builder.add_pair(Rank::from_index(r as u8).unwrap(), 1.0);
        }

        // Suited Ax: A2s+
        for r in 0..=11 {
            builder = builder.add_suited(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Kx: K6s+
        for r in 4..=10 {
            builder = builder.add_suited(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Qx: Q8s+
        for r in 6..=9 {
            builder = builder.add_suited(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Jx: J8s+
        for r in 6..=8 {
            builder = builder.add_suited(Rank::Jack, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Tx: T8s+
        for r in 6..=7 {
            builder = builder.add_suited(Rank::Ten, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited 98s, 87s
        builder = builder.add_suited(Rank::Nine, Rank::Eight, 1.0);
        builder = builder.add_suited(Rank::Eight, Rank::Seven, 1.0);

        // Offsuit Ax: A9o+
        for r in 7..=11 {
            builder = builder.add_offsuit(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Kx: KTo+
        for r in 8..=10 {
            builder = builder.add_offsuit(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Qx: QJo
        builder = builder.add_offsuit(Rank::Queen, Rank::Jack, 1.0);

        builder.build()
    }

    /// Button (BTN) opening range (~45% of hands).
    /// Wide range exploiting positional advantage.
    fn btn_rfi() -> Range {
        let mut builder = RangeBuilder::new();
        
        // Pairs: 22+
        for r in 0..=12 {
            builder = builder.add_pair(Rank::from_index(r as u8).unwrap(), 1.0);
        }

        // Suited Ax: A2s+
        for r in 0..=11 {
            builder = builder.add_suited(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Kx: K2s+
        for r in 0..=10 {
            builder = builder.add_suited(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Qx: Q5s+
        for r in 3..=9 {
            builder = builder.add_suited(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Jx: J7s+
        for r in 5..=8 {
            builder = builder.add_suited(Rank::Jack, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited connectors and 1-gappers down to 54s
        builder = builder.add_suited(Rank::Ten, Rank::Nine, 1.0);
        builder = builder.add_suited(Rank::Ten, Rank::Eight, 1.0);
        builder = builder.add_suited(Rank::Ten, Rank::Seven, 1.0);
        builder = builder.add_suited(Rank::Nine, Rank::Eight, 1.0);
        builder = builder.add_suited(Rank::Nine, Rank::Seven, 1.0);
        builder = builder.add_suited(Rank::Eight, Rank::Seven, 1.0);
        builder = builder.add_suited(Rank::Eight, Rank::Six, 1.0);
        builder = builder.add_suited(Rank::Seven, Rank::Six, 1.0);
        builder = builder.add_suited(Rank::Six, Rank::Five, 1.0);
        builder = builder.add_suited(Rank::Five, Rank::Four, 1.0);

        // Offsuit Ax: A2o+
        for r in 0..=11 {
            builder = builder.add_offsuit(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Kx: K8o+
        for r in 6..=10 {
            builder = builder.add_offsuit(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Qx: Q9o+
        for r in 7..=9 {
            builder = builder.add_offsuit(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Jx: J9o+
        for r in 7..=8 {
            builder = builder.add_offsuit(Rank::Jack, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Tx: T9o
        builder = builder.add_offsuit(Rank::Ten, Rank::Nine, 1.0);

        builder.build()
    }

    /// Small Blind (SB) opening range (~35% of hands).
    /// Tighter than BTN due to being out of position postflop.
    fn sb_rfi() -> Range {
        let mut builder = RangeBuilder::new();
        
        // Pairs: 22+
        for r in 0..=12 {
            builder = builder.add_pair(Rank::from_index(r as u8).unwrap(), 1.0);
        }

        // Suited Ax: A2s+
        for r in 0..=11 {
            builder = builder.add_suited(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Kx: K2s+
        for r in 0..=10 {
            builder = builder.add_suited(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Qx: Q8s+
        for r in 6..=9 {
            builder = builder.add_suited(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Jx: J8s+
        for r in 6..=8 {
            builder = builder.add_suited(Rank::Jack, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited Tx: T8s+
        for r in 6..=7 {
            builder = builder.add_suited(Rank::Ten, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Suited connectors
        builder = builder.add_suited(Rank::Nine, Rank::Eight, 1.0);
        builder = builder.add_suited(Rank::Eight, Rank::Seven, 1.0);
        builder = builder.add_suited(Rank::Seven, Rank::Six, 1.0);
        builder = builder.add_suited(Rank::Six, Rank::Five, 1.0);
        builder = builder.add_suited(Rank::Five, Rank::Four, 1.0);

        // Offsuit Ax: A8o+
        for r in 6..=11 {
            builder = builder.add_offsuit(Rank::Ace, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Kx: K9o+
        for r in 7..=10 {
            builder = builder.add_offsuit(Rank::King, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Qx: Q9o+
        for r in 7..=9 {
            builder = builder.add_offsuit(Rank::Queen, Rank::from_index(r as u8).unwrap(), 1.0);
        }
        // Offsuit Jx: JTo
        builder = builder.add_offsuit(Rank::Jack, Rank::Ten, 1.0);

        builder.build()
    }

    /// Big Blind (BB) defense range.
    /// Wide range, getting good pot odds to call.
    fn bb_defense() -> Range {
        Self::btn_rfi() // Use BTN as a proxy for BB wide defense for now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utg_range_size() {
        let range = PreflopRanges::rfi_range(Position::UTG);
        // ~15% of 1326 combos is ~200 combos.
        let combos = range.combos().len();
        assert!(combos > 100 && combos < 150, "UTG combos: {}", combos);
    }

    #[test]
    fn test_btn_range_size() {
        let range = PreflopRanges::rfi_range(Position::BTN);
        // ~45% of 1326 combos is ~600 combos.
        let combos = range.combos().len();
        assert!(combos > 500 && combos < 700, "BTN combos: {}", combos);
    }

    #[test]
    fn test_range_progression() {
        let utg = PreflopRanges::rfi_range(Position::UTG).combos().len();
        let hj = PreflopRanges::rfi_range(Position::HJ).combos().len();
        let co = PreflopRanges::rfi_range(Position::CO).combos().len();
        let btn = PreflopRanges::rfi_range(Position::BTN).combos().len();

        // Ranges should get wider in later positions
        assert!(utg < hj);
        assert!(hj < co);
        assert!(co < btn);
    }
}
