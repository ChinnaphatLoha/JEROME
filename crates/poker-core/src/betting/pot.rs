#[derive(Debug, Clone, Default)]
pub struct Pot {
    pub main_pot: u64,
    pub side_pots: Vec<SidePot>,
}

#[derive(Debug, Clone, Default)]
pub struct SidePot {
    pub amount: u64,
    pub eligible_players: Vec<u8>,
}

impl Pot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> u64 {
        self.main_pot + self.side_pots.iter().map(|p| p.amount).sum::<u64>()
    }

    pub fn add_to_main(&mut self, amount: u64) {
        self.main_pot += amount;
    }
}
