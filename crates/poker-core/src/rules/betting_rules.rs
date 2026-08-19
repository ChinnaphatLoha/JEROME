pub fn min_raise_size(current_bet: u64, last_raise_size: u64, big_blind: u64) -> u64 {
    let min_increase = if last_raise_size == 0 {
        big_blind
    } else {
        last_raise_size
    };
    current_bet + min_increase
}

pub fn max_bet(stack: u64) -> u64 {
    stack
}
