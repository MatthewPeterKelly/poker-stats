use std::fmt;

use crate::hand_score::HandScore;

#[derive(Default, PartialEq, Debug)]
pub struct AggregateScore {
    pub high_card: u32, // total count (all hands have a high card)
    pub flush: u32,
    pub pair: u32,
    pub two_pair: u32,
    pub three_of_a_kind: u32,
    pub four_of_a_kind: u32,
    pub straight: u32,
    pub full_house: u32,
    pub straight_flush: u32,
}

impl AggregateScore {
    pub fn insert(&mut self, score: &HandScore) {
        self.high_card += 1;
        self.flush += score.flush as u32;
        self.pair += score.pair as u32;
        self.two_pair += score.two_pair as u32;
        self.three_of_a_kind += score.three_of_a_kind as u32;
        self.four_of_a_kind += score.four_of_a_kind as u32;
        self.straight += score.straight as u32;
        self.full_house += score.full_house as u32;
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
        let n_pad_name = "three_of_a_kind:".len();
        let display_member = |name, count| {
            format!(
                "{:<n_pad_name$} {:<n_pad_count$} ({:>7.3}%)",
                name,
                count,
                scale * (count as f64)
            )
        };
        write!(
            f,
            "AggregateScore: ({} hands drawn)\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}\n  {}",
            self.high_card,
            display_member("flush:", self.flush),
            display_member("pair:", self.pair),
            display_member("two_pair:", self.two_pair),
            display_member("three_of_a_kind:", self.three_of_a_kind),
            display_member("four_of_a_kind:", self.four_of_a_kind),
            display_member("straight:", self.straight),
            display_member("full_house:", self.full_house),
            display_member("straight_flush:", self.straight_flush),
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
