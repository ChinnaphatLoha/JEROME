/// Factors influencing a poker decision.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecisionFactor {
    StrongEquity,
    WeakEquity,
    NutAdvantage,
    BlockerEffect,
    FoldEquity,
    PositionAdvantage,
    DrawPotential,
    SPRConsideration,
    PotOdds,
    ImpliedOdds,
    ValueBet,
    Bluff,
    SemiBluff,
}

/// A human-readable explanation of a decision, breaking down the factors involved.
#[derive(Debug, Clone, Default)]
pub struct DecisionExplanation {
    /// List of factors and a text description of how they apply.
    pub factors: Vec<(DecisionFactor, String)>,
    /// The string label of the recommended action.
    pub recommended_action_label: String,
}

impl DecisionExplanation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_factor(&mut self, factor: DecisionFactor, description: impl Into<String>) {
        self.factors.push((factor, description.into()));
    }
}
