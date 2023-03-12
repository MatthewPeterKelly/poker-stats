// Note: Each of the utility files must be added here as a
// module so that it can be properly included in others.
mod aggregate_score;
mod card;
mod deck;
mod hand;
mod hand_score;
mod hand_stats;

use poker_stats::aggregate_score::sample_aggregate_scores;
use crate::card::Card;
use crate::hand::Hand;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;

/// Simple demo for the `poker-stats` crate. For now it
/// does not support any arguments. It will do three things:
/// (1) print out the cards in a sorted deck
/// For each of two randomly drawn hands:
/// (2) print out the hand
/// (3) compute and print the card stats
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

    println!("");
    let five_card_hand = Hand::<5>::draw(&mut rng);
    println!("{five_card_hand}");
    let hand_stats = HandStats::from(&five_card_hand);
    println!("{hand_stats}");
    let hand_score = HandScore::from(&hand_stats);
    println!("{hand_score}");

    println!("");
    let seven_card_hand = Hand::<7>::draw(&mut rng);
    println!("{seven_card_hand}");
    let hand_stats = HandStats::from(&seven_card_hand);
    println!("{hand_stats}");
    let hand_score = HandScore::from(&hand_stats);
    println!("{hand_score}");

    // Now draw N random hands and check the stats!
    println!("");
    // Note: there is probably some way to deduce the type of the RNG here...
    let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, 20_000);
    println!("{scores}")
}
