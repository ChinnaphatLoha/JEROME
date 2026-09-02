use crate::scenario::Scenario;
use poker_core::ActionType;
use poker_core::PokerError;
use poker_decision::config::DecisionConfig;
use poker_decision::decision::DecisionEngine;
use std::time::Instant;

/// The result of running a simulation scenario.
pub struct SimulationResult {
    pub scenario_name: String,
    pub recommended_action: ActionType,
    pub equity: f64,
    pub ev: f64,
    pub duration_us: u64,
}

/// Runs a single scenario and returns its result.
pub fn run_scenario(scenario: &Scenario) -> Result<SimulationResult, PokerError> {
    let config = DecisionConfig::default();
    let engine = DecisionEngine::new(config);

    let start = Instant::now();
    let result = engine.analyze(&scenario.game_state)?;
    let duration = start.elapsed();

    Ok(SimulationResult {
        scenario_name: scenario.name.clone(),
        recommended_action: result.recommended_action,
        equity: result.estimated_equity,
        ev: result.estimated_ev,
        duration_us: duration.as_micros() as u64,
    })
}

/// Runs all predefined scenarios.
pub fn run_all_scenarios() -> Vec<Result<SimulationResult, PokerError>> {
    let mut results = Vec::new();
    let preflop = crate::scenario::preflop_scenarios();
    let postflop = crate::scenario::postflop_scenarios();

    for s in preflop.iter().chain(postflop.iter()) {
        results.push(run_scenario(s));
    }

    results
}
