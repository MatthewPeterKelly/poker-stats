use crate::card::Card;
use crate::hand::Hand;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;
use poker_stats::aggregate_score::sample_aggregate_scores;
use rand::rngs::ThreadRng;

pub fn draw_and_display_hand<const CARD_NUMBER: usize>(mut rng: ThreadRng) {
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

pub fn draw_and_display_hand_wrapper(hands_number: usize, rng: ThreadRng) {

    match hands_number {
        5 => {
            draw_and_display_hand::<5>(rng);
        }
        7 => {
            draw_and_display_hand::<7>(rng);
        }
        _ => {
            println!("Invalid number. Enter either 5 or 7")
        }
    }
}

pub fn sample_and_display_statistics(hands_number: usize, sample_number: u32, mut rng: ThreadRng){
    println!("");

    match hands_number {
        5 => {
            let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, sample_number);
            println!("{scores}")
        }
        7 => {            
            let scores = sample_aggregate_scores::<5, rand::rngs::ThreadRng>(&mut rng, sample_number);
            println!("{scores}")
        }
        _ => {
            println!("Invalid number. Enter either 5 or 7")
        }
    }
}
