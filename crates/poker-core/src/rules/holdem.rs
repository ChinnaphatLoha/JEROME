use crate::game::Street;

pub const HOLE_CARD_COUNT: usize = 2;
pub const MAX_BOARD_CARDS: usize = 5;
pub const DECK_SIZE: usize = 52;
pub const MAX_PLAYERS: usize = 9;
pub const MIN_PLAYERS: usize = 2;
pub const HAND_SIZE: usize = 5;

pub fn board_cards_for_street(street: Street) -> usize {
    street.board_card_count()
}
