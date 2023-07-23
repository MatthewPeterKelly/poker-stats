use crate::hand::Hand;
use crate::hand_score::display_hand_data;
use crate::hand_score::HandData;
use crate::hand_score::HandScore;
use crate::hand_stats::HandStats;
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

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

#[allow(dead_code)]
pub fn sample_aggregate_scores<const N_HAND: usize, R: Rng>(
    rng: &mut R,
    num_samples: u32,
) -> AggregateScore {
    let mut scores = AggregateScore::default();
    for _ in 0..num_samples {
        scores.insert(&HandScore::from(&HandStats::from(&Hand::<N_HAND>::draw(
            rng,
        ))));
    }
    scores
}

pub fn parallel_sample_aggregate_scores<const N_HAND: usize, R: Rng>(
    _rng: &mut R,
    num_samples: u32,
    number_of_threads: u32,
) -> AggregateScore {
    let scores = Arc::new(Mutex::new(AggregateScore::default()));
    let num_samples_perthread = num_samples / number_of_threads;
    let num_samples_remainder = num_samples % number_of_threads;

    (0..number_of_threads).into_par_iter().for_each(|_x| {
        let scores_temp = sample_aggregate_scores::<N_HAND, ThreadRng>(
            &mut rand::thread_rng(),
            num_samples_perthread,
        );
        let scores = Arc::clone(&scores);
        let mut scores_clone = scores.lock().unwrap();
        *scores_clone = scores_temp;
    });

    if num_samples_remainder > 0 {
        let scores_temp = sample_aggregate_scores::<N_HAND, ThreadRng>(
            &mut rand::thread_rng(),
            num_samples_perthread,
        );
        let scores = Arc::clone(&scores);
        let mut scores_clone = scores.lock().unwrap();
        *scores_clone = scores_temp;
    }

    return Arc::into_inner(scores).unwrap().into_inner().unwrap();
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
