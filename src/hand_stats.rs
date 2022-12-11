use crate::card::Card;
use crate::card::Hand;
use crate::card::Rank;
use crate::card::Suit;

use std::fmt;

#[derive(Default)]
pub struct HandStats {
    pub rank_count: [usize; Rank::NUM_RANKS],
    pub suit_count: [usize; Suit::NUM_SUITS],
}

impl HandStats {
    #[allow(dead_code)]
    pub fn new<const N: usize>(hand: &Hand<N>) -> HandStats {
        let mut hand_stats: HandStats = Default::default();
        hand_stats.insert_hand(hand);
        hand_stats
    }

    pub fn insert(&mut self, card: Card) {
        self.rank_count[card.rank().id] += 1;
        self.suit_count[card.suit().id] += 1;
    }

    pub fn insert_hand<const N: usize>(&mut self, hand: &Hand<N>) {
        for card in hand.cards {
            self.insert(card);
        }
    }

    pub fn count_cards(&self) -> usize {
        let mut count = 0;
        for suit_count in self.suit_count {
            count += suit_count;
        }
        count
    }

    #[allow(dead_code)]
    pub fn is_flush(&self) -> bool {
        for suit_count in self.suit_count {
            if suit_count >= 5 {
                return true;
            }
        }
        false
    }
}

/// Utility function to nicely format the counts data for the hand statistics.
/// Bins of zero size are skipped. The remaining bins are formatted using a lambda function.
fn hand_stats_array_string<const N: usize, T>(counts: [usize; N], func: T) -> String
where
    T: Fn(usize) -> String,
{
    let mut suits = format!("");
    for (suit_id, count) in counts.iter().enumerate() {
        if count > &0 {
            let line = format!("[{}]: {}, ", func(suit_id), count);
            suits = suits + &line;
        }
    }
    let suits = suits.trim_end_matches(", ");
    suits.to_string()
}

impl fmt::Display for HandStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suits = hand_stats_array_string(self.suit_count, |id| Suit { id }.to_string());
        let ranks = hand_stats_array_string(self.rank_count, |id| Rank { id }.to_string());
        write!(
            f,
            "HandStats:\n  Count: {}\n  Suits: {suits}\n  Ranks: {ranks}\n  Flush: {}",
            self.count_cards(),
            self.is_flush()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::HandStats;
    use crate::deck::Deck;

    /// Ensure that we can draw a hand from a vector of strings, and
    /// then do some checks on the `is_flush()` utility.
    #[test]
    fn minimal_check_for_hand_stats() {
        let deck = Deck::new();

        let check_flush = |cards, is_flush| {
            let hand = deck.draw_hand(cards);
            assert!(hand.is_some());
            let hand_stats = HandStats::new(&hand.unwrap());
            assert_eq!(hand_stats.is_flush(), is_flush);
        };

        check_flush(&["5♣", "T♣", "8♠", "7♣", "9♦"], false);
        check_flush(&["5♣", "T♣", "8♣", "7♣", "9♣"], true);
    }
}
