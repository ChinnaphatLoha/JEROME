use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Suit {
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

impl Suit {
    pub const COUNT: usize = 4;
    pub const ALL: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    pub fn from_index(index: u8) -> Option<Self> {
        if index < 4 {
            Some(unsafe { std::mem::transmute::<u8, Suit>(index) })
        } else {
            None
        }
    }

    pub fn index(self) -> u8 {
        self as u8
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'c' | 'C' => Some(Suit::Clubs),
            'd' | 'D' => Some(Suit::Diamonds),
            'h' | 'H' => Some(Suit::Hearts),
            's' | 'S' => Some(Suit::Spades),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_roundtrip() {
        for suit in Suit::ALL {
            let c = suit.to_char();
            assert_eq!(Suit::from_char(c), Some(suit));
        }
    }
}
