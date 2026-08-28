pub struct ImpliedOddsCalculator;

impl ImpliedOddsCalculator {
    pub fn calculate_implied_odds(pot: u64, call: u64, effective_stack: u64, equity: f64) -> f64 {
        if equity <= 0.0 {
            return f64::MAX;
        }
        
        let required_total_pot = call as f64 / equity;
        let required_extra = required_total_pot - pot as f64 - call as f64;
        
        if required_extra <= 0.0 {
            0.0 // Call is already profitable based on direct pot odds
        } else if required_extra > effective_stack as f64 {
            f64::INFINITY // Not enough stack to justify the call based on implied odds alone
        } else {
            required_extra
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implied_odds() {
        // Pot is 100, we have to call 50. Total pot after call = 150.
        // We have 20% equity. We need 50 / 0.20 = 250 total pot.
        // We need 250 - 100 - 50 = 100 extra from future streets.
        let extra_needed = ImpliedOddsCalculator::calculate_implied_odds(100, 50, 500, 0.2);
        assert!((extra_needed - 100.0).abs() < f64::EPSILON);
        
        // Not enough stack
        let impossible = ImpliedOddsCalculator::calculate_implied_odds(100, 50, 50, 0.2);
        assert_eq!(impossible, f64::INFINITY);
        
        // Direct pot odds already enough (33% required, we have 40%)
        let profitable = ImpliedOddsCalculator::calculate_implied_odds(100, 50, 500, 0.4);
        assert_eq!(profitable, 0.0);
    }
}
