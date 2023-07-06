use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PokerArgs {
    #[command(subcommand)]
    pub command: Option<ArgsCommands>,

    #[arg(long = "sorted-deck")]
    pub sorted_deck: bool,
}

#[derive(Debug, Subcommand)]
pub enum ArgsCommands {
    /// Draw Hands
    DrawHands(DrawHand),

    /// Get statistics
    Statistics(GetStatistics),
}

#[derive(Debug, Args)]
pub struct DrawHand {
    /// Number of cards
    pub hands_number: usize,
}

#[derive(Debug, Args)]
pub struct GetStatistics {
    /// Number of cards
    pub hands_number: usize,

    /// Number of samples
    pub sample_number: u32,
}
