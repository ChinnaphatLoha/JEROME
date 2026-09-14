use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandCategory {
    HighCard = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HandRank(u32);

impl HandRank {
    pub fn new(category: HandCategory, kickers: &[u8]) -> Self {
        let mut val = (category as u32) << 20;
        let mut shift = 16;
        for &k in kickers.iter().take(5) {
            val |= (k as u32 & 0xF) << shift;
            if shift >= 4 {
                shift -= 4;
            } else {
                break;
            }
        }
        Self(val)
    }

    pub const fn from_value(value: u32) -> Self {
        Self(value)
    }

    pub fn category(&self) -> HandCategory {
        let cat_val = (self.0 >> 20) as u8;
        match cat_val {
            0 => HandCategory::HighCard,
            1 => HandCategory::Pair,
            2 => HandCategory::TwoPair,
            3 => HandCategory::ThreeOfAKind,
            4 => HandCategory::Straight,
            5 => HandCategory::Flush,
            6 => HandCategory::FullHouse,
            7 => HandCategory::FourOfAKind,
            8 => HandCategory::StraightFlush,
            _ => HandCategory::HighCard,
        }
    }

    pub fn kickers(&self) -> Vec<u8> {
        let mut kickers = Vec::new();
        for i in 0..5 {
            let shift = 16 - i * 4;
            let k = ((self.0 >> shift) & 0xF) as u8;
            kickers.push(k);
        }
        kickers
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl fmt::Debug for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandRank({:?}, {:?})", self.category(), self.kickers())
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (val: {})", self.category(), self.value())
    }
}
