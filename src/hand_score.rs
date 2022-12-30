use crate::{deck::Deck, hand::cards_are_unique, hand_stats::HandStats};

use std::fmt;

#[derive(Default, PartialEq, Debug)]
pub struct HandScore {
    pub flush: bool,
    pub pair: bool,
    pub two_pair: bool,
    pub three_of_a_kind: bool,
    pub four_of_a_kind: bool,
    pub straight: bool,
    pub full_house: bool,
    pub straight_flush: bool,
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

/// Check to see if there is a straight. Must work for both
/// five and seven card hands, so it is a bit less optimized
/// than it could be for a strictly five card hand.
#[allow(dead_code)]
pub fn is_straight(hand_stats: &HandStats) -> bool {
    let mut cards_in_straight = 0;
    for count in hand_stats.rank_count {
        if count > 0 {
            cards_in_straight = cards_in_straight + 1;
        } else {
            if cards_in_straight > 0 {
                // We were in a straight... but found a gap.
                return false;
            }
        }
        if cards_in_straight >= 5 {
            // Allow for early exit
            return true;
        }
    }
    cards_in_straight >= 5
}

impl HandScore {
    #[allow(dead_code)]
    pub fn new(hand_stats: &HandStats) -> HandScore {
        let mut hand_scores: HandScore = Default::default();
        hand_scores.flush = is_flush(hand_stats);
        hand_scores.populate_simple_multiples(hand_stats);
        hand_scores.straight = is_straight(hand_stats);
        hand_scores.populate_derived_scores();
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
                4 => self.four_of_a_kind = true,
                _ => (),
            }
        }
    }

    fn populate_derived_scores(&mut self) -> () {
        self.full_house = self.pair && self.three_of_a_kind;
        if self.four_of_a_kind {
            self.three_of_a_kind = true;
        }
        if self.three_of_a_kind {
            self.pair = true;
        }
        self.straight_flush = self.straight && self.flush;
    }
}
impl fmt::Display for HandScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HandScore:\n  flush: {}\n  pair: {}\n  \
            two_pair: {}\n  three_of_a_kind: {}\n  \
            four_of_a_kind: {}\n  straight: {}\n  \
            full_house: {}\n  straight_flush: {}",
            self.flush,
            self.pair,
            self.two_pair,
            self.three_of_a_kind,
            self.four_of_a_kind,
            self.straight,
            self.full_house,
            self.straight_flush
        )
    }
}

// TODO: This function is a bit clunky. There are two problems:
// When it fails, it asserts here, not in the caller, making it hard to
// tell which subtest failed. Additionally, it does most of the work to
// convert from an array of string slices into a score, which sounds like
// a constructor. In rust this is done with the From or Into traits on
// the intermediate types. Syntax looks a bit tricky, but good to learn.
// Probably should implement it for each of the intermediate types.

#[allow(dead_code)]
fn check_hand_score_is_valid<const N: usize>(
    deck: &Deck,
    cards: &[&str; N],
    hand_score_soln: HandScore,
) -> () {
    let hand_opt = deck.draw_hand(cards);
    assert!(hand_opt.is_some());
    let hand = hand_opt.unwrap();
    // Sanity check that the test author gave a valid hand
    assert!(cards_are_unique(&hand));
    let hand_stats = HandStats::from(&hand);
    let hand_score = HandScore::new(&hand_stats);
    assert_eq!(hand_score, hand_score_soln);
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use crate::hand_score::HandScore;

    use super::check_hand_score_is_valid;

    #[test]
    fn five_card_hand_scores() {
        let deck = Deck::new();

        check_hand_score_is_valid(
            &deck,
            &["5♣", "8♣", "8♠", "7♣", "9♦"],
            HandScore {
                pair: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "9♣", "8♣", "7♣", "2♣"],
            HandScore {
                flush: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "4♦", "7♣", "7♦", "4♥"],
            HandScore {
                pair: true,
                two_pair: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "4♦", "7♣", "5♦", "5♥"],
            HandScore {
                pair: true,
                three_of_a_kind: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♦", "9♠", "7♠", "8♦", "6♥"],
            HandScore {
                straight: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["4♦", "5♦", "5♣", "4♣", "5♥"],
            HandScore {
                pair: true,
                three_of_a_kind: true,
                full_house: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["9♥", "7♥", "8♥", "T♥", "J♥"],
            HandScore {
                straight: true,
                flush: true,
                straight_flush: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn seven_card_hand_scores() {
        let deck = Deck::new();

        check_hand_score_is_valid(
            &deck,
            &["5♣", "8♣", "3♣", "8♠", "7♣", "T♥", "9♦"],
            HandScore {
                pair: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "9♣", "8♣", "T♥", "9♦", "7♣", "2♣"],
            HandScore {
                flush: true,
                pair: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "4♦", "7♣", "9♣", "8♣", "7♦", "4♥"],
            HandScore {
                pair: true,
                two_pair: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♣", "4♦", "7♣", "5♦", "9♣", "8♣", "5♥"],
            HandScore {
                pair: true,
                three_of_a_kind: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["5♦", "9♠", "7♠", "8♦", "6♥", "T♦", "J♥"],
            HandScore {
                straight: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["4♦", "5♦", "8♦", "6♥", "5♣", "4♣", "5♥"],
            HandScore {
                pair: true,
                three_of_a_kind: true,
                full_house: true,
                ..Default::default()
            },
        );
        check_hand_score_is_valid(
            &deck,
            &["9♥", "7♥", "8♥", "6♥", "5♣", "T♥", "J♥"],
            HandScore {
                straight: true,
                flush: true,
                straight_flush: true,
                ..Default::default()
            },
        );
    }
}
