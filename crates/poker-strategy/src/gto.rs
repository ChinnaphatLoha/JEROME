/// Calculates Minimum Defense Frequency (MDF).
/// This is the percentage of our range we must defend to prevent 
/// the opponent from profitably bluffing any two cards.
pub fn minimum_defense_frequency(bet_size: u64, pot: u64) -> f64 {
    let pot_f = pot as f64;
    let bet_f = bet_size as f64;
    pot_f / (pot_f + bet_f)
}

/// Calculates the Alpha value (Bluffing frequency).
/// This is how often a bet needs to succeed to break even.
pub fn alpha(bet_size: u64, pot: u64) -> f64 {
    let pot_f = pot as f64;
    let bet_f = bet_size as f64;
    bet_f / (pot_f + bet_f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdf() {
        // Half pot bet
        // MDF = 100 / (100 + 50) = 100 / 150 = 0.666...
        let mdf = minimum_defense_frequency(50, 100);
        assert!((mdf - 0.6666).abs() < 0.001);
    }

    #[test]
    fn test_alpha() {
        // Half pot bet
        // Alpha = 50 / (100 + 50) = 50 / 150 = 0.333...
        let a = alpha(50, 100);
        assert!((a - 0.3333).abs() < 0.001);
    }
}
