use crate::game::Street;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Fold,
    Check,
    Call,
    Bet(u64),
    Raise(u64),
    AllIn(u64),
}

impl ActionType {
    pub fn is_aggressive(&self) -> bool {
        matches!(self, Self::Bet(_) | Self::Raise(_) | Self::AllIn(_))
    }

    pub fn amount(&self) -> u64 {
        match self {
            Self::Fold | Self::Check => 0,
            Self::Call => 0, // usually amount would be contextual, but we can store the committed chips in the record
            Self::Bet(a) | Self::Raise(a) | Self::AllIn(a) => *a,
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fold => write!(f, "Fold"),
            Self::Check => write!(f, "Check"),
            Self::Call => write!(f, "Call"),
            Self::Bet(a) => write!(f, "Bet {}", a),
            Self::Raise(a) => write!(f, "Raise {}", a),
            Self::AllIn(a) => write!(f, "All-In {}", a),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRecord {
    pub player_id: u8,
    pub street: Street,
    pub action: ActionType,
    pub amount: u64,
    pub pot_before: u64,
    pub pot_after: u64,
    pub stack_before: u64,
    pub stack_after: u64,
}
