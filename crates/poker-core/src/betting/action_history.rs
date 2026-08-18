use super::ActionRecord;
use crate::game::Street;

#[derive(Debug, Clone, Default)]
pub struct ActionHistory {
    pub actions: Vec<ActionRecord>,
}

impl ActionHistory {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(&mut self, record: ActionRecord) {
        self.actions.push(record);
    }

    pub fn actions_on_street(&self, street: Street) -> impl Iterator<Item = &ActionRecord> {
        self.actions.iter().filter(move |a| a.street == street)
    }

    pub fn actions_by_player(&self, player_id: u8) -> impl Iterator<Item = &ActionRecord> {
        self.actions
            .iter()
            .filter(move |a| a.player_id == player_id)
    }

    pub fn last_action(&self) -> Option<&ActionRecord> {
        self.actions.last()
    }

    pub fn last_aggressive_action(&self) -> Option<&ActionRecord> {
        self.actions.iter().rev().find(|a| a.action.is_aggressive())
    }

    pub fn street_action_count(&self, street: Street) -> usize {
        self.actions_on_street(street).count()
    }

    pub fn has_bet_or_raise_on_street(&self, street: Street) -> bool {
        self.actions_on_street(street)
            .any(|a| a.action.is_aggressive())
    }

    pub fn iter(&self) -> impl Iterator<Item = &ActionRecord> {
        self.actions.iter()
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
