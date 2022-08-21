use array_init;
use rand::Rng;
use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone)]
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
        // HACK -- this allocates
        let mut card_set = HashSet::new();
        while card_set.len() < N {
            card_set.insert(Card::draw_random_card());
        }

        // HACK -- this is a terrible mess.
        //
        // Proposal:
        //
        // (1) use the hash set as the cannonical "hand" and pre-allocate it.
        // (1a) consider using a BTreeSet instead
        // (2) Create a new "deck" class that has a shuffle method.
        // (2a) Then just create views into that data structure (or copy into a buffer)
        //
        // I suspect that 2a will be faster, depending on the shuffle implementation.
        let hand: [Card; N] = array_init::array_init(|i| card_set.iter().nth(i).unwrap().clone());
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
