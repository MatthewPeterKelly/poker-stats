use crate::card::Card;
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

//////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::hand::Hand;
    use rand::SeedableRng;

    /// Ensure that
    #[test]
    fn unique_cards_in_randomly_drawn_hand_test() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(15234202);

        for trial in 0..1000 {
            let hand = Hand::<7>::draw(&mut rng);
            for i in 1..7 {
                for j in 0..i {
                    assert_ne!(
                        hand.cards[i].id, hand.cards[j].id,
                        "trial: {trial}, i: {i}, j: {j}, left: {}, right: {}",
                        hand.cards[i], hand.cards[j]
                    );
                }
            }
        }
    }
}
