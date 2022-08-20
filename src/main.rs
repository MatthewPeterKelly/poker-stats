pub struct Card {
    index: i32,
}

impl Card {
    /// Returns the suit of this [`Card`].
    fn suit(&self) -> i32 {
        self.index % 4
    }
}

fn main() {
    let card = Card { index: 9 };
    println!("Hello, world! Your card suit is: {}", card.suit());
}
