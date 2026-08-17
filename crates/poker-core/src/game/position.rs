use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    UTG,
    UTG1,
    MP,
    MP1,
    HJ,
    CO,
    BTN,
    SB,
    BB,
}

impl Position {
    pub fn is_early(&self) -> bool {
        matches!(self, Self::UTG | Self::UTG1)
    }

    pub fn is_middle(&self) -> bool {
        matches!(self, Self::MP | Self::MP1 | Self::HJ)
    }

    pub fn is_late(&self) -> bool {
        matches!(self, Self::CO | Self::BTN)
    }

    pub fn is_blind(&self) -> bool {
        matches!(self, Self::SB | Self::BB)
    }

    pub fn positions_for_table_size(size: usize) -> Vec<Position> {
        match size {
            2 => vec![Self::BTN, Self::BB],
            3 => vec![Self::BTN, Self::SB, Self::BB],
            4 => vec![Self::UTG, Self::BTN, Self::SB, Self::BB],
            5 => vec![Self::UTG, Self::CO, Self::BTN, Self::SB, Self::BB],
            6 => vec![Self::UTG, Self::HJ, Self::CO, Self::BTN, Self::SB, Self::BB],
            7 => vec![
                Self::UTG,
                Self::UTG1,
                Self::HJ,
                Self::CO,
                Self::BTN,
                Self::SB,
                Self::BB,
            ],
            8 => vec![
                Self::UTG,
                Self::UTG1,
                Self::MP,
                Self::HJ,
                Self::CO,
                Self::BTN,
                Self::SB,
                Self::BB,
            ],
            9 => vec![
                Self::UTG,
                Self::UTG1,
                Self::MP,
                Self::MP1,
                Self::HJ,
                Self::CO,
                Self::BTN,
                Self::SB,
                Self::BB,
            ],
            _ => vec![],
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::UTG => "UTG",
            Self::UTG1 => "UTG1",
            Self::MP => "MP",
            Self::MP1 => "MP1",
            Self::HJ => "HJ",
            Self::CO => "CO",
            Self::BTN => "BTN",
            Self::SB => "SB",
            Self::BB => "BB",
        };
        write!(f, "{}", s)
    }
}
