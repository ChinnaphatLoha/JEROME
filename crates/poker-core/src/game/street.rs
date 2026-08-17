use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl Street {
    pub fn board_card_count(&self) -> usize {
        match self {
            Self::Preflop => 0,
            Self::Flop => 3,
            Self::Turn => 4,
            Self::River => 5,
        }
    }

    pub fn is_preflop(&self) -> bool {
        matches!(self, Self::Preflop)
    }

    pub fn is_postflop(&self) -> bool {
        !self.is_preflop()
    }
}

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preflop => write!(f, "Preflop"),
            Self::Flop => write!(f, "Flop"),
            Self::Turn => write!(f, "Turn"),
            Self::River => write!(f, "River"),
        }
    }
}
