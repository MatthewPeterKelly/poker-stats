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
}

fn draw_five_cards() -> [Card; 5] {
    // let hand = [Card; 5];
    // for i in 0..5 {
    //     hand[i] = Card {
    //         id: rand::thread_rng().gen_range(0..51),
    //     };
    // }
    // hand

    // https://docs.rs/array-init/latest/array_init/#  neat
    let hand: [Card; 5] = array_init::array_init(|i: usize| Card {
        id: rand::thread_rng().gen_range(0..51),
    });
}

fn main() {
    for id in 0..52 {
        let card = Card { id };
        println!("Sorted:  Suit: {}, Index: {}", card.suit(), card.index());
    }
    for _ in 0..10 {
        let card = Card {
            id: rand::thread_rng().gen_range(0..51),
        };
        println!("Random:  Suit: {}, Index: {}", card.suit(), card.index());
    }
}
