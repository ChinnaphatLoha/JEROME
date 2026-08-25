use super::model::OpponentProfile;
use crate::range::Range;

// Placeholder for an Action enum. In a full system this would come from `poker-core::betting::Action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Fold,
    Call,
    Raise,
    Bet,
    Check,
}

pub struct RangeUpdater;

impl RangeUpdater {
    pub fn update_range(range: &mut Range, action: ActionType, profile: &OpponentProfile) {
        // Very basic stub: apply a multiplier to the entire range based on action.
        // A real bayesian update would analyze the exact combination and scale its weight
        // individually depending on how likely that combo takes the given action.

        let multiplier = match action {
            ActionType::Fold => 0.0,
            ActionType::Check => 1.0,
            ActionType::Call => 1.0,
            ActionType::Bet | ActionType::Raise => profile.pfr / profile.vpip.max(0.01),
        };

        for combo in range.combos_mut() {
            combo.weight *= multiplier;
        }

        range.normalize();
    }
}

pub fn update_range(range: &mut Range, action: ActionType, profile: &OpponentProfile) {
    RangeUpdater::update_range(range, action, profile);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::range::RangeBuilder;
    use poker_core::card::Rank;

    #[test]
    fn test_update_range() {
        let mut range = RangeBuilder::new().add_pair(Rank::Ace, 1.0).build();
        let profile = OpponentProfile::default_unknown();
        update_range(&mut range, ActionType::Raise, &profile);

        // After normalization, weights should still be 1.0 / combos.len() for remaining ones
        // if they are updated equally, unless they hit 0.
        assert!(range.total_weight() > 0.99 && range.total_weight() < 1.01);
    }
}
