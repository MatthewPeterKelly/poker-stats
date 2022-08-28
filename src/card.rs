use array_init;
use rand::Rng;
use std::fmt;

#[derive(PartialEq, PartialOrd)]
pub struct Card {
    pub id: i32,
}

impl Card {
    /// Returns the suit (club, spade, heart, diamond) of this [`Card`],
    /// where the suit is represented as an integer.
    pub fn suit(&self) -> i32 {
        self.id % 4
    }

    /// Returns the "index" (ace, two, three, ... jack, queen, king) of the [`Card`].
    pub fn index(&self) -> i32 {
        self.id / 4
    }

    pub fn draw_random_card<R: Rng>(rng: &mut R) -> Card {
        Card {
            id: rng.gen_range(0..51),
        }
    }

    /// Check for duplicates in the array, starting with `start_index`.
    /// Return the index of the first duplicate found.
    /// The `start_index` parameter allows the search to resume after replacing
    /// a duplicate entry.
    fn check_for_duplicates<const N: usize>(
        start_index: usize,
        cards: &[Card; N],
    ) -> Option<usize> {
        for i in start_index..N {
            for j in 0..i {
                if cards[i] == cards[j] {
                    return Some(i);
                }
            }
        }
        return None;
    }

    /// Returns an array of N cards that are sampled from the deck without replacement
    /// Note: this algorithm is efficient for small N, but is very slow as N approaches
    /// 51. For values larger than 51 it will block forever. For now, this is private
    /// to the module so that it can only be called when N << 52.
    fn draw_without_replacement<const N: usize, R: Rng>(rng: &mut R) -> [Card; N] {
        // Draw N cards with replacement
        let mut hand: [Card; N] = array_init::array_init(|_| Card::draw_random_card(rng));
        // Replace any duplicates.
        let mut start_index = 1;
        while let Some(i) = Card::check_for_duplicates(start_index, &hand) {
            hand[i] = Card::draw_random_card(rng);
            start_index = i;
        }
        hand
    }

    pub fn draw_five_cards<R: Rng>(rng: &mut R) -> [Card; 5] {
        Card::draw_without_replacement::<5, R>(rng)
    }

    pub fn draw_seven_cards<R: Rng>(rng: &mut R) -> [Card; 7] {
        Card::draw_without_replacement::<7, R>(rng)
    }

    pub fn suit_to_string(&self) -> String {
        match self.suit() {
            0 => String::from("♣"),
            1 => String::from("♦"),
            2 => String::from("♥"),
            3 => String::from("♠"),
            _ => String::from("?"),
        }
    }

    pub fn index_to_string(&self) -> String {
        let one_based_index = self.index() + 1;
        match one_based_index {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            _ => one_based_index.to_string(),
        }
    }
}

/// Return owned string representing the card. Examples:
/// K♥ A♣ Q♥ 3♦ 7 8♠ 2♥
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.index_to_string(), self.suit_to_string())
    }
}

#[cfg(test)]
mod tests {

    use crate::card::Card;
    use rand::SeedableRng;

    /// Ensure that
    #[test]
    fn unique_cards_in_randomly_drawn_hand_test() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(15234202);

        for trial in 0..1000 {
            let cards = Card::draw_seven_cards(&mut rng);
            for i in 1..7 {
                for j in 0..i {
                    assert_ne!(
                        cards[i].id, cards[j].id,
                        "trial: {trial}, i: {i}, j: {j}, left: {}, right: {}",
                        cards[i], cards[j]
                    );
                }
            }
        }
    }
}
