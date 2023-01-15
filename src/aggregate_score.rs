use std::fmt;

use crate::hand_score::display_hand_data;
use crate::hand_score::HandData;
use crate::hand_score::HandScore;

pub type AggregateScore = HandData<u32>;

impl AggregateScore {
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
