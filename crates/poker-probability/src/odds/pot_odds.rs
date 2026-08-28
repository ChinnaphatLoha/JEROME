pub struct PotOddsCalculator;

impl PotOddsCalculator {
    pub fn pot_odds(call: u64, pot_after_call: u64) -> f64 {
        if pot_after_call == 0 {
            return 0.0;
        }
        call as f64 / pot_after_call as f64
    }

    pub fn required_equity(call: u64, pot_after_call: u64) -> f64 {
        Self::pot_odds(call, pot_after_call)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pot_odds() {
        assert_eq!(PotOddsCalculator::pot_odds(50, 150), 1.0 / 3.0);
        assert_eq!(PotOddsCalculator::required_equity(50, 150), 1.0 / 3.0);
    }
}
