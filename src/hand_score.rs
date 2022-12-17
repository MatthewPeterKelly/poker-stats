use crate::hand_stats::HandStats;

use std::fmt;

#[derive(Default)]
pub struct HandScore {
    pub flush: bool,
    pub pair: bool,
    pub two_pair: bool,
    pub three_of_a_kind: bool,
}

#[allow(dead_code)]
pub fn is_flush(hand_stats: &HandStats) -> bool {
    for count in hand_stats.suit_count {
        if count >= 5 {
            return true;
        }
    }
    false
}

impl HandScore {
    #[allow(dead_code)]
    pub fn new(hand_stats: &HandStats) -> HandScore {
        let mut hand_scores: HandScore = Default::default();
        hand_scores.flush = is_flush(hand_stats);
        hand_scores.populate_simple_multiples(hand_stats);
        hand_scores
    }

    fn populate_simple_multiples(&mut self, hand_stats: &HandStats) -> () {
        for count in hand_stats.rank_count {
            match count {
                2 => {
                    if self.pair {
                        self.two_pair = true;
                    } else {
                        self.pair = true;
                    }
                }
                3 => self.three_of_a_kind = true,
                _ => (),
            }
        }
    }
}
impl fmt::Display for HandScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HandScore:\n  flush: {}\n  pair: {}\n  two_pair: {}\n  three_of_a_kind: {}",
            self.flush, self.pair, self.two_pair, self.three_of_a_kind
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use crate::hand_score::is_flush;
    use crate::hand_stats::HandStats;

    /// Ensure that we can draw a hand from a vector of strings, and
    /// then do some checks on the `is_flush()` utility.
    #[test]
    fn minimal_check_for_hand_stats() {
        let deck = Deck::new();

        let check_flush = |cards, is_flush_soln| {
            let hand = deck.draw_hand(cards);
            assert!(hand.is_some());
            let hand_stats = HandStats::new(&hand.unwrap());
            assert_eq!(is_flush(&hand_stats), is_flush_soln);
        };

        check_flush(&["5♣", "T♣", "8♠", "7♣", "9♦"], false);
        check_flush(&["5♣", "T♣", "8♣", "7♣", "9♣"], true);
    }
}
