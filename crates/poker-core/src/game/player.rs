use super::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerStatus {
    Active,
    Folded,
    AllIn,
    Eliminated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub id: u8,
    pub position: Position,
    pub stack: u64,
    pub status: PlayerStatus,
    pub bet_this_round: u64,
}

impl Player {
    pub fn is_active(&self) -> bool {
        self.status == PlayerStatus::Active
    }

    pub fn is_all_in(&self) -> bool {
        self.status == PlayerStatus::AllIn
    }

    pub fn effective_stack(&self, other_stack: u64) -> u64 {
        std::cmp::min(self.stack, other_stack)
    }
}
