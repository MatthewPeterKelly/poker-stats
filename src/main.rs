mod card;

use crate::card::Card;
use crate::card::Hand;

/// Simple demo for the `poker-stats` crate. For now it
/// does not support any arguments. It will do three things:
/// (1) print out the cards in a sorted deck
/// (2) print out a randomly drawn 5-card hand
/// (3) print out a randomly drawn 7-card hand
fn main() {
    let mut rng = rand::thread_rng();

    println!("Sorted Deck:");
    for id in 0..52 {
        let card = Card { id };
        if card.suit().id == 3 {
            println!("  {}", card);
        } else {
            print!("  {}  ", card);
        }
    }
    println!();

    print!("Five Cards: ");
    let five_card_hand = Hand::<5>::draw(&mut rng);
    for card in five_card_hand.cards {
        print!("{} ", card);
    }
    println!();

    print!("Seven Cards: ");
    let seven_card_hand = Hand::<7>::draw(&mut rng);
    for card in seven_card_hand.cards {
        print!("{} ", card);
    }
    println!();
}
