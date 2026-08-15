use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokerError {
    InvalidCard { index: u8 },
    InvalidRank { value: u8 },
    InvalidSuit { value: u8 },
    DuplicateCard { card: u8 },
    CardNotAvailable { card: u8 },
    InvalidBoard { reason: &'static str },
    InvalidBet { reason: &'static str },
    InvalidStack { reason: &'static str },
    InvalidAction { reason: &'static str },
    InvalidGameState { reason: &'static str },
    InsufficientCards { needed: usize, available: usize },
}

impl fmt::Display for PokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCard { index } => write!(f, "Invalid card index: {}", index),
            Self::InvalidRank { value } => write!(f, "Invalid rank value: {}", value),
            Self::InvalidSuit { value } => write!(f, "Invalid suit value: {}", value),
            Self::DuplicateCard { card } => write!(f, "Duplicate card: {}", card),
            Self::CardNotAvailable { card } => write!(f, "Card not available: {}", card),
            Self::InvalidBoard { reason } => write!(f, "Invalid board: {}", reason),
            Self::InvalidBet { reason } => write!(f, "Invalid bet: {}", reason),
            Self::InvalidStack { reason } => write!(f, "Invalid stack: {}", reason),
            Self::InvalidAction { reason } => write!(f, "Invalid action: {}", reason),
            Self::InvalidGameState { reason } => write!(f, "Invalid game state: {}", reason),
            Self::InsufficientCards { needed, available } => {
                write!(
                    f,
                    "Insufficient cards: needed {}, available {}",
                    needed, available
                )
            }
        }
    }
}

impl std::error::Error for PokerError {}
