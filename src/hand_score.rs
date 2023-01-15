use crate::hand::Hand;
use crate::{deck::Deck, hand::cards_are_unique, hand_stats::HandStats};

use std::fmt;

#[derive(Default, PartialEq, Debug)]
pub struct HandData<T> {
    pub high_card: T,
    pub pair: T,
    pub two_pair: T,
    pub three_of_a_kind: T,
    pub straight: T,
    pub flush: T,
    pub full_house: T,
    pub four_of_a_kind: T,
    pub straight_flush: T,
}

pub type HandScore = HandData<bool>;

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

pub fn display_hand_data<T, F>(hand_data: &HandData<T>, object_name: &str, value_fmt: F) -> String
where
    T: fmt::Display,
    T: Copy,
    F: Fn(T) -> String,
{
    let n_pad_name = "three_of_a_kind:".len();
    let display_member = |name, value| format!("{:<n_pad_name$}  {}", name, value);
    format!(
        "{}: \n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
        object_name,
        display_member("high_card", value_fmt(hand_data.high_card)),
        display_member("pair", value_fmt(hand_data.pair)),
        display_member("two_pair", value_fmt(hand_data.two_pair)),
        display_member("three_of_a_kind", value_fmt(hand_data.three_of_a_kind)),
        display_member("straight", value_fmt(hand_data.straight)),
        display_member("flush", value_fmt(hand_data.flush)),
        display_member("full_house", value_fmt(hand_data.full_house)),
        display_member("four_of_a_kind", value_fmt(hand_data.four_of_a_kind)),
        display_member("straight_flush", value_fmt(hand_data.straight_flush)),
    )
}

impl fmt::Display for HandScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            display_hand_data(&self, "HandScore", |value| { value.to_string() })
        )
    }
}

impl From<&HandStats> for HandScore {
    #[allow(dead_code)]
    fn from(hand_stats: &HandStats) -> HandScore {
        let mut hand_scores: HandScore = Default::default();
        hand_scores.high_card = true;
        hand_scores.flush = is_flush(hand_stats);
        hand_scores.populate_simple_multiples(hand_stats);
        hand_scores.straight = is_straight(hand_stats);
        hand_scores.populate_derived_scores();
        hand_scores
    }
}

impl<const N: usize> From<&Hand<N>> for HandScore {
    #[allow(dead_code)]
    fn from(hand: &Hand<N>) -> HandScore {
        HandScore::from(&HandStats::from(hand))
    }
}

#[allow(dead_code)]
fn card_names_to_hand_score<const N: usize>(deck: &Deck, cards: &[&str; N]) -> HandScore {
    let hand_opt = deck.draw_hand(cards);
    // Check that the card names can be parsed:
    assert!(hand_opt.is_some());
    let hand = hand_opt.unwrap();
    // Check that the test author gave a valid hand
    assert!(cards_are_unique(&hand));
    HandScore::from(&hand)
}

#[cfg(test)]
mod tests {
    use crate::deck::Deck;
    use crate::hand_score::card_names_to_hand_score;
    use crate::hand_score::HandScore;

    #[test]
    fn five_card_hand_scores() {
        let deck = Deck::new();

        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "8♣", "8♠", "7♣", "9♦"]),
            HandScore {
                pair: true,
                high_card: true,
                ..Default::default()
            }
        );

        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "9♣", "8♣", "7♣", "2♣"]),
            HandScore {
                flush: true,
                high_card: true,
                ..Default::default()
            }
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "4♦", "7♣", "7♦", "4♥"]),
            HandScore {
                pair: true,
                two_pair: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "4♦", "7♣", "5♦", "5♥"]),
            HandScore {
                pair: true,
                three_of_a_kind: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♦", "9♠", "7♠", "8♦", "6♥"]),
            HandScore {
                straight: true,
                high_card: true,
                ..Default::default()
            }
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["4♦", "5♦", "5♣", "4♣", "5♥"]),
            HandScore {
                pair: true,
                three_of_a_kind: true,
                full_house: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["9♥", "7♥", "8♥", "T♥", "J♥"]),
            HandScore {
                straight: true,
                flush: true,
                straight_flush: true,
                high_card: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn seven_card_hand_scores() {
        let deck = Deck::new();

        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "8♣", "3♣", "8♠", "7♣", "T♥", "9♦"]),
            HandScore {
                pair: true,
                high_card: true,
                ..Default::default()
            }
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "9♣", "8♣", "T♥", "9♦", "7♣", "2♣"]),
            HandScore {
                flush: true,
                pair: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "4♦", "7♣", "9♣", "8♣", "7♦", "4♥"]),
            HandScore {
                pair: true,
                two_pair: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♣", "4♦", "7♣", "5♦", "9♣", "8♣", "5♥"]),
            HandScore {
                pair: true,
                three_of_a_kind: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["5♦", "9♠", "7♠", "8♦", "6♥", "T♦", "J♥"]),
            HandScore {
                straight: true,
                high_card: true,
                ..Default::default()
            }
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["4♦", "5♦", "8♦", "6♥", "5♣", "4♣", "5♥"]),
            HandScore {
                pair: true,
                three_of_a_kind: true,
                full_house: true,
                high_card: true,
                ..Default::default()
            },
        );
        assert_eq!(
            card_names_to_hand_score(&deck, &["9♥", "7♥", "8♥", "6♥", "5♣", "T♥", "J♥"]),
            HandScore {
                straight: true,
                flush: true,
                straight_flush: true,
                high_card: true,
                ..Default::default()
            },
        );
    }
}
