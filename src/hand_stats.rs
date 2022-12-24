use crate::card::Card;
use crate::card::Hand;
use crate::card::Rank;
use crate::card::Suit;
use itertools::Itertools;

use std::fmt;

#[derive(Default, PartialEq, Debug)]
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
            "HandStats:\n  Count: {}\n  Suits: {suits}\n  Ranks: {ranks}",
            self.count_cards()
        )
    }
}

#[allow(dead_code)]
pub fn cards_are_unique<const N: usize>(hand: &Hand<N>) -> bool {
    hand.cards.into_iter().unique().count() == N
}

// TODO:  consider adding some tests here that the stats are correct...
// Maybe covered by the hand score tests?
