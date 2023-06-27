use clap:: {
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PokerArgs{
    // Option command to run the program
    #[arg(short = 'r', long = "run")]
    /// Run program
    pub run_option: bool,
    
    // Optional argument to get the number of cards for playing
    // Set to default so that if one doesn't any value it goes to demo mode
    #[arg(required = false, default_value = "default")]
    /// Number of cards.
    pub cards_number: String,

    // Optional argument to get the number of samples if required
    // Set to default for demo mode
    #[arg(required = false, default_value_t = 20_000)]
    /// Number of samples.
    pub samples_number: u32,
}