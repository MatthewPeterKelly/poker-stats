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

fn main() {
    for id in 0..52 {
        let card = Card { id };
        println!("Suit: {}, Index: {}", card.suit(), card.index());
    }
}
