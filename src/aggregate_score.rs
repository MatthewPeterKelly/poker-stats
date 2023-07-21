use rand::rngs::StdRng;
use rand::Rng;
use std::fmt;
use std::sync::Mutex;
use std::thread;

use crate::hand::Hand;
use crate::hand_score::display_hand_data;
use crate::hand_score::HandData;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;

pub type AggregateScore = HandData<u32>;

impl AggregateScore {
    #[allow(dead_code)]
    pub fn insert(&mut self, score: &HandScore) {
        self.high_card += 1;
        self.pair += score.pair as u32;
        self.two_pair += score.two_pair as u32;
        self.three_of_a_kind += score.three_of_a_kind as u32;
        self.straight += score.straight as u32;
        self.flush += score.flush as u32;
        self.full_house += score.full_house as u32;
        self.four_of_a_kind += score.four_of_a_kind as u32;
        self.straight_flush += score.straight_flush as u32;
    }
}

impl fmt::Display for AggregateScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = if self.high_card > 0 {
            100.0 / (self.high_card as f64)
        } else {
            0.0
        };
        let n_pad_count = self.high_card.to_string().len();
        let display_member =
            |count| format!("{:<n_pad_count$} ({:>7.3}%)", count, scale * (count as f64));
        write!(
            f,
            "{}",
            display_hand_data(&self, "HandScore", display_member)
        )
    }
}

fn sample<const N_HAND: usize, R: Rng>(
    rng: &Mutex<R>,
    aggregate: &Mutex<AggregateScore>,
    samples: u32,
) {
    for _ in 0..samples {
        let hand = Hand::<N_HAND>::draw(&mut *rng.lock().unwrap());
        let score = HandScore::from(&HandStats::from(&hand));
        aggregate.lock().unwrap().insert(&score);
    }
}

#[allow(dead_code)]
pub fn sample_aggregate_scores<const N_HAND: usize>(
    rng: &mut StdRng,
    samples: u32,
    threads: u32,
) -> AggregateScore {
    let scores = Mutex::new(AggregateScore::default());
    let rng = Mutex::new(rng);

    let samples_per_thread = samples / threads;
    let remainder = samples % threads;

    // using scoped threads to automatically wait for everything to finish,
    // which also allows us to use borrowed data instead of Arc
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| sample::<N_HAND, _>(&rng, &scores, samples_per_thread));
        }

        if remainder > 0 {
            s.spawn(|| sample::<N_HAND, _>(&rng, &scores, remainder));
        }
    });

    scores.into_inner().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::aggregate_score::AggregateScore;
    use crate::hand_score::HandScore;

    #[test]
    fn basic_operation() {
        let mut scores = AggregateScore::default();

        scores.insert(&HandScore {
            pair: true,
            two_pair: true,
            ..Default::default()
        });

        scores.insert(&HandScore {
            pair: true,
            three_of_a_kind: true,
            full_house: true,
            ..Default::default()
        });

        assert_eq!(
            scores,
            AggregateScore {
                high_card: 2,
                pair: 2,
                two_pair: 1,
                three_of_a_kind: 1,
                full_house: 1,
                ..Default::default()
            }
        );

        println!("{scores}")
    }
}
