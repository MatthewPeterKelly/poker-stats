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
    pub fn insert(&mut self, score: &HandData<u32>) {
        self.high_card += score.high_card;
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

pub fn parallel_sample_aggregate_scores<const N_HAND: usize>(
    num_samples: u32,
    num_threads: u32,
) -> AggregateScore {
    let scores = Arc::new(Mutex::new(AggregateScore::default()));
    let num_samples_per_thread = num_samples / num_threads;
    let num_samples_remainder = num_samples % num_threads;
    let mut sample_sizes = vec![num_samples_per_thread; num_threads as usize];
    if num_samples_remainder > 0 {
        sample_sizes.insert(num_threads as usize, num_samples_remainder)
    }

    (0..sample_sizes.len()).into_par_iter().for_each(|x| {
        let scores_temp =
            sample_aggregate_scores::<N_HAND, ThreadRng>(&mut rand::thread_rng(), sample_sizes[x]);
        // Creates clone of scores for handling the change
        let scores = Arc::clone(&scores);
        // Locks score for thread to enter data. Keeps it locked till it is completed with writing process.
        let mut scores_clone = scores.lock().unwrap();
        // Finally adds the data
        scores_clone.insert(&scores_temp);
    });

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
            high_card: true as u32,
            pair: true as u32,
            two_pair: true as u32,
            ..Default::default()
        });

        scores.insert(&HandScore {
            high_card: true as u32,
            pair: true as u32,
            three_of_a_kind: true as u32,
            full_house: true as u32,
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
