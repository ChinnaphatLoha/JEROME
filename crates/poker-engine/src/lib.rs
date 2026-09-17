pub use poker_analysis as analysis;
pub use poker_core as core;
pub use poker_decision as decision;
pub use poker_probability as probability;
pub use poker_strategy as strategy;

use poker_core::error::PokerError;
use poker_core::game::GameState;
use poker_decision::config::DecisionConfig;
use poker_decision::decision::{DecisionEngine, DecisionResult};

#[derive(Default)]
pub struct EngineConfig {
    pub decision_config: DecisionConfig,
}

pub struct PokerEngine {
    config: EngineConfig,
    engine: DecisionEngine,
}

impl PokerEngine {
    pub fn new(config: EngineConfig) -> Self {
        let engine = DecisionEngine::new(config.decision_config.clone());
        Self { config, engine }
    }

    pub fn analyze(&self, state: &GameState) -> Result<DecisionResult, PokerError> {
        self.engine.analyze(state)
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }
}
