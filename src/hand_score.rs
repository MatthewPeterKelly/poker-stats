use crate::hand_stats::HandStats;

use std::fmt;

#[derive(Default)]
pub struct HandScore {
    pub is_flush: bool,
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
    pub fn new<const N: usize>(hand_stats: &HandStats) -> HandScore {
        let mut hand_scores: HandScore = Default::default();
        hand_scores.is_flush = is_flush(hand_stats);
        hand_scores
    }
}
impl fmt::Display for HandScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandScore:\n  is_flush: {}\n", self.is_flush)
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
