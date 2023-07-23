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

use crate::args::{CommandsEnum, PokerArgs, StatisticsSampleParameters};
use clap::Parser;
use output::{draw_and_display_hand_wrapper, print_sorted_deck, sample_and_display_statistics};
/// Simple demo for the `poker-stats` crate. For now it
/// does not support any arguments. It will do three things:
/// (1) print out the cards in a sorted deck
/// For each of two randomly drawn hands:
/// (2) print out the hand
/// (3) compute and print the card stats
fn main() {
    let args: PokerArgs = PokerArgs::parse();
    let rng = rand::thread_rng();
    // Matching the command
    match &args.command {
        Some(CommandsEnum::DrawHand { hands_size }) => {
            draw_and_display_hand_wrapper(*hands_size, rng)
        }
        Some(CommandsEnum::Statistics(StatisticsSampleParameters {
            hands_number,
            number_of_samples,
            number_of_threads,
        })) => sample_and_display_statistics(
            *hands_number,
            *number_of_samples,
            rng,
            *number_of_threads,
        ),
        Some(CommandsEnum::SortedDeck) => print_sorted_deck(),
        None => draw_and_display_hand_wrapper(5, rng),
    }
}
