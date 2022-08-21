use array_init;
use rand::Rng;
use std::fmt;

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

    pub fn draw_random_card() -> Card {
        Card {
            id: rand::thread_rng().gen_range(0..51),
        }
    }

    /// Returns an array of N cards that are sampled from the deck without replacement
    /// Note: this algorithm is efficient for small N, but is very slow as N approaches
    /// 51. For values larger than 51 it will block forever.
    fn draw_without_replacement<const N: usize>() -> [Card; N] {
        // Draw N cards with replacement
        let mut hand: [Card; N] = array_init::array_init(|_| Card::draw_random_card());
        // Replace any duplicates.
        for i in 1..N {
            for j in 0..i {
                while hand[j].id == hand[i].id {
                    // There is a bug here!
                    // What happens if the card we draw is ALSO a match?
                    //
                    // The fix: we need to break this out into a utility function
                    // that finds a card that doesn't match any in a slice.
                    //
                    // UGH: this actually is non-trivial
                    // https://timvieira.github.io/blog/post/2019/09/16/algorithms-for-sampling-without-replacement/
                    //
                    // New plan: how rare is it to just pull a valid hand?
                    //
                    // >>> (1) * (51/52.) * (50 / 52.) * (49 / 52.) * (48 /52.)
                    // 0.8202837785791814
                    // >>> (1) * (51/52.) * (50 / 52.) * (49 / 52.) * (48 /52.) * (47 / 52.) * (46 / 52.)
                    // 0.6558629916006621
                    //
                    hand[i] = Card::draw_random_card();
                }
            }
        }
        hand
    }

    pub fn draw_five_cards() -> [Card; 5] {
        Card::draw_without_replacement::<5>()
    }

    pub fn draw_seven_cards() -> [Card; 7] {
        Card::draw_without_replacement::<7>()
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

    /// Ensure that
    #[test]
    fn unique_cards_in_randomly_drawn_hand_test() {
        for trial in 0..1000 {
            let cards = Card::draw_seven_cards();

            println!("");
            for my_card in cards.iter() {
                print!("{} ", my_card);
            }
            println!("");

            for i in 1..7 {
                for j in 0..i {
                    println!("i: {i}, j: {j} ");

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
