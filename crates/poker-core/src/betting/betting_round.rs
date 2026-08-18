use super::ActionType;
use crate::game::GameState;

pub fn legal_actions(state: &GameState) -> Vec<ActionType> {
    let hero = state.hero();
    let to_call = state.to_call();
    let stack = hero.stack;

    let mut actions = vec![ActionType::Fold];

    if to_call == 0 {
        actions.push(ActionType::Check);
        if stack > 0 {
            actions.push(ActionType::Bet(std::cmp::min(state.min_raise, stack))); // Min bet
            actions.push(ActionType::AllIn(stack));
        }
    } else {
        if stack >= to_call {
            actions.push(ActionType::Call);
            let min_raise_total = state.current_bet + state.min_raise;
            let raise_amount_to_add = min_raise_total.saturating_sub(hero.bet_this_round);
            if stack > raise_amount_to_add {
                actions.push(ActionType::Raise(min_raise_total)); // Min raise amount
            }
        }
        if stack > 0 {
            actions.push(ActionType::AllIn(stack));
        }
    }

    actions
}
