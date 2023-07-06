// Note: Each of the utility files must be added here as a
// module so that it can be properly included in others.
mod aggregate_score;
mod args;
mod card;
mod deck;
mod hand;
mod hand_score;
mod hand_stats;

use crate::args::{ArgsCommands, DrawHand, PokerArgs};
use crate::card::Card;
use crate::hand::Hand;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;
use args::GetStatistics;
use clap::Parser;
use poker_stats::aggregate_score::sample_aggregate_scores;
use rand::rngs::ThreadRng;

/// Simple demo for the `poker-stats` crate. For now it
/// does not support any arguments. It will do three things:
/// (1) print out the cards in a sorted deck
/// For each of two randomly drawn hands:
/// (2) print out the hand
/// (3) compute and print the card stats
fn main() {
    let args: PokerArgs = PokerArgs::parse();

    // Matching the command
    match &args.command {
        // For draw-hand command
        Some(ArgsCommands::DrawHands(DrawHand { hands_number })) => {
            // If call draw-hands with hands_number as 5
            if *hands_number == 5 {
                output_specific::<5>(rand::thread_rng(), 20000);
            }
            // If call draw-hands with hands_number as 7
            else if *hands_number == 7 {
                output_specific::<7>(rand::thread_rng(), 20000);
            }
            // If hand_number is not 5 not 7
            else {
                println!("Invalid number. Enter 5 or 7")
            }
        }

        // For statistics command
        Some(ArgsCommands::Statistics(GetStatistics {
            hands_number,
            sample_number,
        })) => {
            if *hands_number == 5 {
                output_specific::<5>(rand::thread_rng(), *sample_number);
            } else if *hands_number == 7 {
                output_specific::<7>(rand::thread_rng(), *sample_number);
            } else {
                println!("Invalid number. Enter 5 or 7")
            }
        }

        None => {
            // If neither sorted_deck option is called nor any other i.e. for cargo run command (Default run)
            if !(args.sorted_deck) {
                output_specific::<5>(rand::thread_rng(), 20000);
            }
        }
    }

    if args.sorted_deck {
        println!("Sorted Deck:");

        for id in 0..52 {
            let card = Card { id };

            if card.suit().id == 3 {
                println!("  {}", card);
            } else {
                print!("  {}  ", card);
            }
        }
    }
}

pub fn output_specific<const CARD_NUMBER: usize>(mut rng: ThreadRng, sample_number: u32) {
    println!("");
    let card_hand = Hand::<CARD_NUMBER>::draw(&mut rng);
    println!("{card_hand}");
    let hand_stats = HandStats::from(&card_hand);
    println!("{hand_stats}");
    let hand_score = HandScore::from(&hand_stats);
    println!("{hand_score}");

    // Now draw N random hands and check the stats!
    println!("");
    // Note: there is probably some way to deduce the type of the RNG here...
    let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, sample_number);
    println!("{scores}")
}
