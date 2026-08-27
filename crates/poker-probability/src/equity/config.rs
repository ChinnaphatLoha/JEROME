#[derive(Debug, Clone)]
pub struct EquityConfig {
    pub exact_threshold: usize,
    pub mc_samples: usize,
    pub seed: Option<u64>,
}

impl Default for EquityConfig {
    fn default() -> Self {
        Self {
            exact_threshold: 10000,
            mc_samples: 10000,
            seed: None,
        }
    }
}
