use poker_probability::equity::EquityConfig;

/// Configuration for generating candidate actions.
#[derive(Debug, Clone)]
pub struct ActionAbstractionConfig {
    /// Fractions of the pot to consider for betting (e.g., 0.33, 0.5, 0.75)
    pub bet_sizes: Vec<f64>,
    /// Multipliers of the previous bet/raise to consider for raising (e.g., 2.0, 2.5, 3.0)
    pub raise_sizes: Vec<f64>,
}

impl Default for ActionAbstractionConfig {
    fn default() -> Self {
        Self {
            bet_sizes: vec![0.33, 0.5, 0.66, 0.75, 1.0],
            raise_sizes: vec![2.0, 2.5, 3.0],
        }
    }
}

/// Overall configuration for the decision engine.
#[derive(Debug, Clone, Default)]
pub struct DecisionConfig {
    /// Configuration for action abstractions.
    pub action_config: ActionAbstractionConfig,
    /// Configuration for equity calculations.
    pub equity_config: EquityConfig,
}
