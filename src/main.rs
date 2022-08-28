mod card;

use crate::card::Card;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Sorted Deck:");
    for id in 0..52 {
        let card = Card { id };
        if card.suit() == 3 {
            println!("  {}", card);
        } else {
            print!("  {}  ", card);
        }
    }
    println!("");

    print!("Five Cards: ");
    for my_card in Card::draw_five_cards(&mut rng) {
        print!("{} ", my_card);
    }
    println!("");

    print!("Seven Cards: ");
    for my_card in Card::draw_seven_cards(&mut rng) {
        print!("{} ", my_card);
    }
    println!("");
}
