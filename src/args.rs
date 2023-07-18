use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PokerArgs {
    #[command(subcommand)]
    pub command: Option<CommandsEnum>,
}

#[derive(Debug, Subcommand)]
pub enum CommandsEnum {
    /// Draw random hand, display, and score
    DrawHand{hands_size: usize,},

    /// Compute statistics on a 5/7 card hand with N samples
    Statistics(StatisticsSampleParameters),

    /// To print sorted deck
    SortedDeck,
}

#[derive(Debug, Args)]
pub struct StatisticsSampleParameters {
    /// Number of cards
    pub hands_number: usize,

    pub number_of_samples: u32,
}

