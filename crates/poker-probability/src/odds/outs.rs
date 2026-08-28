use poker_analysis::hand::draw::DrawInfo;

pub struct OutsCalculator;

impl OutsCalculator {
    pub fn count_outs(draw_info: &DrawInfo) -> u8 {
        let mut outs = 0;
        
        // Simple disjoint outs approximation
        if draw_info.is_flush_draw && draw_info.is_oesd {
            outs = 15;
        } else if draw_info.is_flush_draw && draw_info.is_gutshot {
            outs = 12;
        } else if draw_info.is_flush_draw {
            outs = 9;
        } else if draw_info.is_oesd {
            outs = 8;
        } else if draw_info.is_gutshot {
            outs = 4;
        }
        
        // Overcards usually give 3 outs each (discounting slightly due to not being pure nut outs)
        // Here we just add 3 per overcard
        outs += draw_info.overcards * 3;
        
        // Cap outs at 21 to avoid crazy estimates
        if outs > 21 { 
            21 
        } else {
            outs
        }
    }

    pub fn equity_from_outs(outs: u8, streets_remaining: u8) -> f64 {
        let outs_f = outs as f64;
        let p = match streets_remaining {
            1 => outs_f * 2.0 / 100.0,
            2 => outs_f * 4.0 / 100.0,
            _ => 0.0,
        };
        p.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_outs() {
        let mut info = DrawInfo {
            is_flush_draw: true,
            is_nut_flush_draw: false,
            is_oesd: false,
            is_gutshot: false,
            overcards: 0,
        };
        assert_eq!(OutsCalculator::count_outs(&info), 9);
        
        info.is_oesd = true;
        assert_eq!(OutsCalculator::count_outs(&info), 15);
    }

    #[test]
    fn test_equity_from_outs() {
        assert_eq!(OutsCalculator::equity_from_outs(9, 1), 0.18);
        assert_eq!(OutsCalculator::equity_from_outs(9, 2), 0.36);
    }
}
