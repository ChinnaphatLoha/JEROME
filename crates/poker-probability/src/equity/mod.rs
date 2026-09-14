pub mod config;
pub mod exact;
pub mod monte_carlo;
pub mod multiway;

pub use config::EquityConfig;
pub use exact::calculate_exact_equity;
pub use monte_carlo::calculate_mc_equity;
pub use multiway::{calculate_multiway_equity, MultiWayEquityResult};

#[derive(Debug, Clone, PartialEq)]
pub struct EquityResult {
    pub win: f64,
    pub tie: f64,
    pub loss: f64,
    pub equity: f64,
    pub samples: usize,
    pub method: &'static str,
}
