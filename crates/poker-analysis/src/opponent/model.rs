#[derive(Debug, Clone, PartialEq)]
pub struct OpponentProfile {
    pub vpip: f64,
    pub pfr: f64,
    pub three_bet: f64,
    pub fold_to_cbet: f64,
}

impl OpponentProfile {
    pub fn new(vpip: f64, pfr: f64, three_bet: f64, fold_to_cbet: f64) -> Self {
        Self {
            vpip,
            pfr,
            three_bet,
            fold_to_cbet,
        }
    }

    pub fn default_unknown() -> Self {
        Self {
            vpip: 0.25,
            pfr: 0.18,
            three_bet: 0.08,
            fold_to_cbet: 0.45,
        }
    }
}
