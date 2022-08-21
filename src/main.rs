use array_init;
use rand::Rng;

pub struct Card {
    id: i32,
}

impl Card {
    /// Returns the suit (club, spade, heart, diamond) of this [`Card`],
    /// where the suit is represented as an integer.
    fn suit(&self) -> i32 {
        self.id % 4
    }

    /// Returns the "index" (ace, two, three, ... jack, queen, king) of the [`Card`].
    fn index(&self) -> i32 {
        self.id / 4
    }

    fn draw_random_card() -> Card {
        Card {
            id: rand::thread_rng().gen_range(0..51),
        }
    }
}

fn draw_without_replacement<const N: usize>() -> [Card; N] {
    // Draw N cards with replacement
    let mut hand: [Card; N] = array_init::array_init(|_| Card::draw_random_card());
    // Replace any duplicates.
    for i in 1..N {
        for j in 0..i {
            if hand[j].id == hand[i].id {
                hand[i] = Card::draw_random_card();
            }
        }
    }
    hand
}

fn draw_five_cards() -> [Card; 5] {
    draw_without_replacement::<5>()
}

fn draw_seven_cards() -> [Card; 7] {
    draw_without_replacement::<7>()
}

fn main() {
    for id in 0..52 {
        let card = Card { id };
        println!("Sorted:  Suit: {}, Index: {}", card.suit(), card.index());
    }
    for card in draw_five_cards() {
        println!(
            "Five Cards:  Suit: {}, Index: {}",
            card.suit(),
            card.index()
        );
    }
    for card in draw_seven_cards() {
        println!(
            "Seven Cards:  Suit: {}, Index: {}",
            card.suit(),
            card.index()
        );
    }
}
