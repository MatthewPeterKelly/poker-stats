use array_init;
use rand::Rng;
use std::fmt;

pub struct Card {
    id: i32,
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

fn main() {
    for id in 0..52 {
        let card = Card { id };
        println!("Sorted:  {}", card);
    }

    print!("Five Cards: ");
    for card in Card::draw_five_cards() {
        print!("{} ", card);
    }
    println!("");

    print!("Seven Cards: ");
    for card in Card::draw_seven_cards() {
        print!("{} ", card);
    }
    println!("");
}
