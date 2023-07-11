use crate::card::Card;
use crate::hand::Hand;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;
use poker_stats::aggregate_score::sample_aggregate_scores;
use rand::rngs::ThreadRng;

pub fn compute<const CARD_NUMBER: usize>(mut rng: ThreadRng) {
    println!("");
    let card_hand = Hand::<CARD_NUMBER>::draw(&mut rng);
    println!("{card_hand}");
    let hand_stats = HandStats::from(&card_hand);
    println!("{hand_stats}");
    let hand_score = HandScore::from(&hand_stats);
    println!("{hand_score}");
}

pub fn print_sorted_deck(){
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

pub fn draw_hands(hands_number: usize) {

    match hands_number {
        5 => {
            compute::<5>(rand::thread_rng());
        }
        7 => {
            compute::<7>(rand::thread_rng());
        }
        _ => {
            println!("Invalid number. Enter 5 or 7")
        }
    }
}

pub fn getstatistics(hands_number: usize, sample_number: u32){
    
    match hands_number {
        5 => {
            let mut rng = rand::thread_rng();
            println!("");
            let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, sample_number);
            println!("{scores}")
        }
        7 => {
            let mut rng = rand::thread_rng();
            println!("");
            let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, sample_number);
            println!("{scores}")
        }
        _ => {
            println!("Invalid number. Enter 5 or 7")
        }
    }
}
