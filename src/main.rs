use array_init;
use rand::Rng;

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
}

fn main() {
    for id in 0..52 {
        let card = Card { id };
        println!("Sorted:  Suit: {}, Index: {}", card.suit(), card.index());
    }
    for card in Card::draw_five_cards() {
        println!(
            "Five Cards:  Suit: {}, Index: {}",
            card.suit(),
            card.index()
        );
    }
    for card in Card::draw_seven_cards() {
        println!(
            "Seven Cards:  Suit: {}, Index: {}",
            card.suit(),
            card.index()
        );
    }
}
