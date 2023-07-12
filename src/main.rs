// Note: Each of the utility files must be added here as a
// module so that it can be properly included in others.
mod aggregate_score;
mod args;
mod card;
mod deck;
mod hand;
mod hand_score;
mod hand_stats;
mod output;

use crate::args::{ PokerArgs, StatisticsSampleParameters, CommandsEnum };
use clap::Parser;
use output::{draw_hand, print_sorted_deck, getstatistics};
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
        Some(CommandsEnum::DrawHand{ hands_number}) => 
            draw_hand(*hands_number),
        Some(CommandsEnum::Statistics(StatisticsSampleParameters { hands_number, sample_number })) => 
            getstatistics(*hands_number, *sample_number),
        Some(CommandsEnum::SortedDeck) => 
            print_sorted_deck(),
        None => draw_hand(5),
    }
}
