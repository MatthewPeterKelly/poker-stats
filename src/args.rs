use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PokerArgs{
    #[arg(short = 'r', long = "run")]

    /// Run program
    pub run_option: bool,
    /// Number of cards.
    pub cards_number: String,
    /// Number of samples.
    pub samples_number: String,
}