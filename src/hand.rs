use crate::card::Card;
use itertools::Itertools;
use rand::Rng;
use std::fmt;

#[derive(PartialEq, Debug, Hash)]
pub struct Hand<const N: usize> {
    pub cards: [Card; N],
}

impl<const N: usize> Hand<N> {
    /// Check for duplicates in the array, starting with `start_index`.
    /// Return the index of the first duplicate found.
    /// The `start_index` parameter allows the search to resume after replacing
    /// a duplicate entry.
    fn check_for_duplicates(&self, start_index: usize) -> Option<usize> {
        for i in start_index..N {
            for j in 0..i {
                if self.cards[i] == self.cards[j] {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Returns an array of N cards that are sampled from the deck without
    /// replacement Note: this algorithm is efficient for small N, but is very
    /// slow as N approaches Card::NUM_CARDS. For values larger than
    /// (Card::NUM_CARDS-1) it will block forever. For now, this is private to
    /// the module so that it can only be called when N << Card::NUM_CARDS.
    pub fn draw<R: Rng>(rng: &mut R) -> Hand<N> {
        // Draw N cards with replacement
        let mut hand = Hand {
            cards: array_init::array_init(|_| Card::draw_random_card(rng)),
        };
        // Replace any duplicates.
        let mut start_index: usize = 1;
        while let Some(i) = hand.check_for_duplicates(start_index) {
            hand.cards[i] = Card::draw_random_card(rng);
            start_index = i;
        }
        hand
    }
}

impl<const N: usize> fmt::Display for Hand<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = self
            .cards
            .iter()
            .map(|x| x.to_string() + ", ")
            .collect::<String>();
        let string = string.trim_end_matches(", ");
        write!(f, "Hand: {string}")
    }
}

#[allow(dead_code)]
pub fn cards_are_unique<const N: usize>(hand: &Hand<N>) -> bool {
    // There are probably more efficient ways to do this for small N, but this
    // is certainly the least typing, and fast enough for simple unit tests.
    hand.cards.into_iter().unique().count() == N
}

//////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::hand::cards_are_unique;
    use crate::hand::Hand;
    use rand::SeedableRng;

    /// Ensure that cards within a single hand are unique.
    #[test]
    fn unique_cards_in_randomly_drawn_hand_test() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(15234202);

        for _ in 0..800 {
            let hand = Hand::<7>::draw(&mut rng);
            assert!(cards_are_unique(&hand));
            let hand = Hand::<5>::draw(&mut rng);
            assert!(cards_are_unique(&hand));
        }
    }
}
