use super::{Player, Street};
use crate::betting::ActionHistory;
use crate::card::{Card, Deck};
use crate::error::PokerError;

#[derive(Debug, Clone)]
pub struct GameState {
    pub street: Street,
    pub players: Vec<Player>,
    pub hero_index: usize,
    pub hero_cards: [Card; 2],
    pub board: Vec<Card>,
    pub pot: u64,
    pub current_bet: u64,
    pub min_raise: u64,
    pub dealer_index: usize,
    pub action_history: ActionHistory,
    pub dead_cards: Vec<Card>,
    pub big_blind: u64,
}

impl GameState {
    pub fn hero(&self) -> &Player {
        &self.players[self.hero_index]
    }

    pub fn hero_stack(&self) -> u64 {
        self.hero().stack
    }

    pub fn effective_stack(&self) -> u64 {
        let hero_stack = self.hero_stack();
        self.active_players()
            .filter(|p| p.id != self.hero().id)
            .map(|p| p.stack)
            .max()
            .map(|max_opp_stack| std::cmp::min(hero_stack, max_opp_stack))
            .unwrap_or(0)
    }

    pub fn spr(&self) -> f64 {
        if self.pot == 0 {
            0.0
        } else {
            self.effective_stack() as f64 / self.pot as f64
        }
    }

    pub fn active_players(&self) -> impl Iterator<Item = &Player> {
        self.players.iter().filter(|p| p.is_active())
    }

    pub fn active_opponent_count(&self) -> usize {
        self.active_players()
            .filter(|p| p.id != self.hero().id)
            .count()
    }

    pub fn all_known_cards(&self) -> Vec<Card> {
        let mut cards = Vec::new();
        cards.extend_from_slice(&self.hero_cards);
        cards.extend_from_slice(&self.board);
        cards.extend_from_slice(&self.dead_cards);
        cards
    }

    pub fn available_deck(&self) -> Deck {
        let mut deck = Deck::full();
        deck.remove_all(&self.all_known_cards()).unwrap_or(());
        deck
    }

    pub fn to_call(&self) -> u64 {
        self.current_bet.saturating_sub(self.hero().bet_this_round)
    }

    pub fn validate(&self) -> Result<(), PokerError> {
        if self.board.len() != self.street.board_card_count() {
            return Err(PokerError::InvalidBoard {
                reason: "Board card count does not match street",
            });
        }

        let mut seen = std::collections::HashSet::new();
        for &card in self.all_known_cards().iter() {
            if !seen.insert(card.index()) {
                return Err(PokerError::DuplicateCard { card: card.index() });
            }
        }

        if self.hero_index >= self.players.len() {
            return Err(PokerError::InvalidGameState {
                reason: "Hero index out of bounds",
            });
        }

        if self.players.len() < 2 {
            return Err(PokerError::InvalidGameState {
                reason: "At least 2 players required",
            });
        }

        Ok(())
    }
}

pub struct GameStateBuilder {
    street: Street,
    players: Vec<Player>,
    hero_index: usize,
    hero_cards: Option<[Card; 2]>,
    board: Vec<Card>,
    pot: u64,
    current_bet: u64,
    min_raise: u64,
    dealer_index: usize,
    action_history: ActionHistory,
    dead_cards: Vec<Card>,
    big_blind: u64,
}

impl Default for GameStateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GameStateBuilder {
    pub fn new() -> Self {
        Self {
            street: Street::Preflop,
            players: Vec::new(),
            hero_index: 0,
            hero_cards: None,
            board: Vec::new(),
            pot: 0,
            current_bet: 0,
            min_raise: 0,
            dealer_index: 0,
            action_history: ActionHistory::default(),
            dead_cards: Vec::new(),
            big_blind: 0,
        }
    }

    pub fn street(mut self, s: Street) -> Self {
        self.street = s;
        self
    }

    pub fn hero_index(mut self, i: usize) -> Self {
        self.hero_index = i;
        self
    }

    pub fn hero_cards(mut self, cards: [Card; 2]) -> Self {
        self.hero_cards = Some(cards);
        self
    }

    pub fn board(mut self, cards: Vec<Card>) -> Self {
        self.board = cards;
        self
    }

    pub fn pot(mut self, p: u64) -> Self {
        self.pot = p;
        self
    }

    pub fn current_bet(mut self, b: u64) -> Self {
        self.current_bet = b;
        self
    }

    pub fn min_raise(mut self, r: u64) -> Self {
        self.min_raise = r;
        self
    }

    pub fn big_blind(mut self, bb: u64) -> Self {
        self.big_blind = bb;
        self
    }

    pub fn dealer_index(mut self, d: usize) -> Self {
        self.dealer_index = d;
        self
    }

    pub fn add_player(mut self, player: Player) -> Self {
        self.players.push(player);
        self
    }

    pub fn dead_cards(mut self, cards: Vec<Card>) -> Self {
        self.dead_cards = cards;
        self
    }

    pub fn build(self) -> Result<GameState, PokerError> {
        let hero_cards = self.hero_cards.ok_or(PokerError::InvalidGameState {
            reason: "Missing hero cards",
        })?;

        let state = GameState {
            street: self.street,
            players: self.players,
            hero_index: self.hero_index,
            hero_cards,
            board: self.board,
            pot: self.pot,
            current_bet: self.current_bet,
            min_raise: self.min_raise,
            dealer_index: self.dealer_index,
            action_history: self.action_history,
            dead_cards: self.dead_cards,
            big_blind: self.big_blind,
        };

        state.validate()?;
        Ok(state)
    }
}
