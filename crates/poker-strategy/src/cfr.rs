use std::collections::HashMap;

/// A simplified Counterfactual Regret Minimization (CFR) engine for
/// computing approximate Nash equilibrium strategies in abstracted
/// poker decision trees.
///
/// This implements vanilla CFR over a finite, explicitly constructed
/// game tree with fold/call/raise actions. The strategy profile
/// converges toward a Nash equilibrium as iterations increase.
///
/// # Theory
///
/// For each information set (decision point), CFR tracks:
/// - **Cumulative regret** per action: how much utility was lost by not
///   choosing that action in past iterations.
/// - **Cumulative strategy** per action: the running sum of the strategies
///   played, used to compute the average strategy at convergence.
///
/// At each iteration the current strategy is proportional to positive
/// regrets (regret matching). The *average* strategy across all
/// iterations converges to a Nash equilibrium.

/// Available actions at a CFR decision node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CfrAction {
    Fold,
    Call,
    Raise,
}

impl CfrAction {
    pub const ALL: [CfrAction; 3] = [CfrAction::Fold, CfrAction::Call, CfrAction::Raise];
}

/// A node in the abstracted game tree.
#[derive(Debug, Clone)]
pub enum CfrNode {
    /// A player decision point, identified by an info-set key.
    Decision {
        player: u8,
        info_set: String,
        children: HashMap<CfrAction, Box<CfrNode>>,
    },
    /// A terminal node with a known payoff for Player 0 (Hero).
    Terminal {
        payoff: f64,
    },
    /// A chance node that branches uniformly over its children.
    Chance {
        children: Vec<Box<CfrNode>>,
    },
}

// ... InfoSetData remains same ...

/// Accumulated regret and strategy sums for one information set.
#[derive(Debug, Clone)]
struct InfoSetData {
    cumulative_regret: HashMap<CfrAction, f64>,
    cumulative_strategy: HashMap<CfrAction, f64>,
    num_actions: usize,
}

impl InfoSetData {
    fn new(actions: &[CfrAction]) -> Self {
        let mut regret = HashMap::new();
        let mut strategy = HashMap::new();
        for &a in actions {
            regret.insert(a, 0.0);
            strategy.insert(a, 0.0);
        }
        Self {
            cumulative_regret: regret,
            cumulative_strategy: strategy,
            num_actions: actions.len(),
        }
    }

    /// Regret matching: current strategy proportional to positive regrets.
    fn current_strategy(&self, realization_weight: f64) -> HashMap<CfrAction, f64> {
        let mut strategy = HashMap::new();
        let mut normalizing_sum = 0.0;

        for (&action, &regret) in &self.cumulative_regret {
            let positive_regret = regret.max(0.0);
            strategy.insert(action, positive_regret);
            normalizing_sum += positive_regret;
        }

        for (_, prob) in strategy.iter_mut() {
            if normalizing_sum > 0.0 {
                *prob /= normalizing_sum;
            } else {
                *prob = 1.0 / self.num_actions as f64;
            }
        }

        let _ = realization_weight;

        strategy
    }

    /// Returns the average strategy (the convergent Nash approximation).
    fn average_strategy(&self) -> HashMap<CfrAction, f64> {
        let mut avg = HashMap::new();
        let total: f64 = self.cumulative_strategy.values().sum();
        for (&action, &sum) in &self.cumulative_strategy {
            if total > 0.0 {
                avg.insert(action, sum / total);
            } else {
                avg.insert(action, 1.0 / self.num_actions as f64);
            }
        }
        avg
    }
}

/// The simplified CFR solver.
#[derive(Debug)]
pub struct CfrSolver {
    info_sets: HashMap<String, InfoSetData>,
    iterations: usize,
}

/// Result of CFR training: the average strategy for each information set.
#[derive(Debug, Clone)]
pub struct CfrResult {
    /// Maps info-set key → (action → probability).
    pub strategies: HashMap<String, HashMap<CfrAction, f64>>,
    /// Number of training iterations completed.
    pub iterations: usize,
}

impl CfrSolver {
    pub fn new() -> Self {
        Self {
            info_sets: HashMap::new(),
            iterations: 0,
        }
    }

    /// Trains the strategy profile by running `iterations` of CFR on `root`.
    pub fn train(&mut self, root: &CfrNode, iterations: usize) -> CfrResult {
        for _ in 0..iterations {
            self.cfr_traverse(root, 1.0, 1.0, 0); // start assuming P0 acts if chance
            self.iterations += 1;
        }

        let mut strategies = HashMap::new();
        for (key, data) in &self.info_sets {
            strategies.insert(key.clone(), data.average_strategy());
        }

        CfrResult {
            strategies,
            iterations: self.iterations,
        }
    }

    /// Recursive CFR traversal. Returns the expected utility for the
    /// traversing player at this node.
    fn cfr_traverse(
        &mut self,
        node: &CfrNode,
        p0_reach: f64,
        p1_reach: f64,
        player_to_eval: u8,
    ) -> f64 {
        match node {
            CfrNode::Terminal { payoff } => {
                if player_to_eval == 0 {
                    *payoff
                } else {
                    -*payoff
                }
            }

            CfrNode::Chance { children } => {
                if children.is_empty() {
                    return 0.0;
                }
                let p = 1.0 / children.len() as f64;
                let mut ev = 0.0;
                for child in children {
                    // Chance node doesn't belong to any player, just pass through reaches.
                    ev += p * self.cfr_traverse(child, p0_reach, p1_reach, player_to_eval);
                }
                ev
            }

            CfrNode::Decision {
                player,
                info_set,
                children,
            } => {
                let current_player = *player;
                let reach_prob = if current_player == 0 { p0_reach } else { p1_reach };
                let opp_reach_prob = if current_player == 0 { p1_reach } else { p0_reach };

                let actions: Vec<CfrAction> = children.keys().cloned().collect();
                if !self.info_sets.contains_key(info_set) {
                    self.info_sets
                        .insert(info_set.clone(), InfoSetData::new(&actions));
                }

                let strategy = self
                    .info_sets
                    .get(info_set)
                    .unwrap()
                    .current_strategy(reach_prob);

                // Accumulate strategy sum weighted by reach probability.
                {
                    let data = self.info_sets.get_mut(info_set).unwrap();
                    for (&action, &prob) in &strategy {
                        *data
                            .cumulative_strategy
                            .entry(action)
                            .or_insert(0.0) += reach_prob * prob;
                    }
                }

                let mut action_utilities: HashMap<CfrAction, f64> = HashMap::new();
                let mut node_utility = 0.0;

                for (&action, child) in children {
                    let action_prob = strategy.get(&action).copied().unwrap_or(0.0);
                    
                    let next_p0_reach = if current_player == 0 { p0_reach * action_prob } else { p0_reach };
                    let next_p1_reach = if current_player == 1 { p1_reach * action_prob } else { p1_reach };

                    // Utility of the child for the *current* player.
                    let child_util = self.cfr_traverse(
                        child,
                        next_p0_reach,
                        next_p1_reach,
                        current_player,
                    );
                    action_utilities.insert(action, child_util);
                    node_utility += action_prob * child_util;
                }

                // Update cumulative regrets.
                {
                    let data = self.info_sets.get_mut(info_set).unwrap();
                    for (&action, &util) in &action_utilities {
                        let regret = util - node_utility;
                        *data
                            .cumulative_regret
                            .entry(action)
                            .or_insert(0.0) += opp_reach_prob * regret;
                    }
                }

                // Return utility from the perspective of player_to_eval
                if player_to_eval == current_player {
                    node_utility
                } else {
                    -node_utility
                }
            }
        }
    }
}

impl Default for CfrSolver {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Helpers for building game trees ───────────────────────────────

/// Builds a simple Kuhn-poker-style toy tree for testing CFR convergence.
///
/// Game: 1-card poker. Pot starts at 2 (each player antes 1).
/// Player 1 acts first: Check or Bet(1).
///   - If Check → Player 2: Check (showdown) or Bet(1).
///     - If Player 2 Bets → Player 1: Fold or Call(1).
///   - If Bet → Player 2: Fold or Call(1).
pub fn build_kuhn_tree(hero_card_rank: u8, opp_card_rank: u8) -> CfrNode {
    let showdown_payoff = if hero_card_rank > opp_card_rank {
        1.0
    } else if hero_card_rank < opp_card_rank {
        -1.0
    } else {
        0.0
    };

    let showdown_bet_payoff = if hero_card_rank > opp_card_rank {
        2.0
    } else if hero_card_rank < opp_card_rank {
        -2.0
    } else {
        0.0
    };

    // Player 1 decision
    let mut p1_children = HashMap::new();

    // Player 1 checks
    let mut p2_after_check = HashMap::new();
    // Player 2 checks back → showdown
    p2_after_check.insert(
        CfrAction::Call, // "Check" represented as Call
        Box::new(CfrNode::Terminal {
            payoff: showdown_payoff,
        }),
    );
    // Player 2 bets
    let mut p1_facing_bet = HashMap::new();
    p1_facing_bet.insert(
        CfrAction::Fold,
        Box::new(CfrNode::Terminal { payoff: -1.0 }),
    );
    p1_facing_bet.insert(
        CfrAction::Call,
        Box::new(CfrNode::Terminal {
            payoff: showdown_bet_payoff,
        }),
    );
    p2_after_check.insert(
        CfrAction::Raise,
        Box::new(CfrNode::Decision {
            player: 0,
            info_set: format!("P1:{}:cb", hero_card_rank),
            children: p1_facing_bet,
        }),
    );

    p1_children.insert(
        CfrAction::Call, // "Check"
        Box::new(CfrNode::Decision {
            player: 1,
            info_set: format!("P2:{}:c", opp_card_rank),
            children: p2_after_check,
        }),
    );

    // Player 1 bets
    let mut p2_facing_bet = HashMap::new();
    p2_facing_bet.insert(
        CfrAction::Fold,
        Box::new(CfrNode::Terminal { payoff: 1.0 }),
    );
    p2_facing_bet.insert(
        CfrAction::Call,
        Box::new(CfrNode::Terminal {
            payoff: showdown_bet_payoff,
        }),
    );

    p1_children.insert(
        CfrAction::Raise, // "Bet"
        Box::new(CfrNode::Decision {
            player: 1,
            info_set: format!("P2:{}:b", opp_card_rank),
            children: p2_facing_bet,
        }),
    );

    CfrNode::Decision {
        player: 0,
        info_set: format!("P1:{}:", hero_card_rank),
        children: p1_children,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cfr_terminal_payoff() {
        let mut solver = CfrSolver::new();
        let node = CfrNode::Terminal { payoff: 5.0 };
        let ev = solver.cfr_traverse(&node, 1.0, 1.0, 0);
        assert!((ev - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_cfr_kuhn_converges() {
        // Run CFR over all 6 possible card deals of Kuhn poker
        // (3 cards: 1, 2, 3 — each player gets one).
        let mut solver = CfrSolver::new();

        let deals = vec![
            (1u8, 2u8),
            (1, 3),
            (2, 1),
            (2, 3),
            (3, 1),
            (3, 2),
        ];

        for _ in 0..10_000 {
            for &(hero_card, opp_card) in &deals {
                let tree = build_kuhn_tree(hero_card, opp_card);
                solver.cfr_traverse(&tree, 1.0, 1.0, 0);
            }
            solver.iterations += 1;
        }

        // Extract average strategies.
        let mut strategies = HashMap::new();
        for (key, data) in &solver.info_sets {
            strategies.insert(key.clone(), data.average_strategy());
        }

        // In Kuhn poker Nash equilibrium:
        // - Player 1 with card 1: should check with probability 1 (never bet as a bluff... or bet 1/3).
        // - Player 1 with card 3: should bet with high probability.
        let p1_card3 = strategies.get("P1:3:").unwrap();
        let raise_prob = p1_card3.get(&CfrAction::Raise).unwrap_or(&0.0);
        // Player 1 with card 3 should bet frequently (the optimal is about 3x the bluff frequency).
        assert!(
            *raise_prob > 0.5,
            "P1 with card 3 should bet frequently, got: {}",
            raise_prob
        );

        // Player 2 with card 3 facing a bet should always call.
        let p2_card3_b = strategies.get("P2:3:b").unwrap();
        let call_prob = p2_card3_b.get(&CfrAction::Call).unwrap_or(&0.0);
        assert!(
            *call_prob > 0.90,
            "P2 with card 3 facing bet should call, got: {}",
            call_prob
        );
    }

    #[test]
    fn test_cfr_strategy_sums_to_one() {
        let mut solver = CfrSolver::new();
        let deals = vec![(1u8, 2u8), (2, 1)];

        for _ in 0..1000 {
            for &(h, o) in &deals {
                let tree = build_kuhn_tree(h, o);
                solver.cfr_traverse(&tree, 1.0, 1.0, 0);
            }
            solver.iterations += 1;
        }

        for (key, data) in &solver.info_sets {
            let avg = data.average_strategy();
            let sum: f64 = avg.values().sum();
            assert!(
                (sum - 1.0).abs() < 0.01,
                "Strategy for {} doesn't sum to 1.0: {} (strategies: {:?})",
                key,
                sum,
                avg
            );
        }
    }
}
