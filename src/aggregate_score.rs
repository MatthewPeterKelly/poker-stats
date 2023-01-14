use std::fmt;

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

impl fmt::Display for AggregateScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = if self.high_card > 0 {
            100.0 / (self.high_card as f64)
        } else {
            0.0
        };
        write!(
            f,
            "AggregateScore:\n  high_card: {} ({}%)\n  flush: {} ({}%)\n  pair: {} ({}%)\n  \
            two_pair: {} ({}%)\n  three_of_a_kind: {} ({}%)\n  \
            four_of_a_kind: {} ({}%)\n  straight: {} ({}%)\n  \
            full_house: {} ({}%)\n  straight_flush: {} ({}%)",
            self.high_card,
            (self.high_card as f64) * scale,
            self.flush,
            (self.flush as f64) * scale,
            self.pair,
            (self.pair as f64) * scale,
            self.two_pair,
            (self.two_pair as f64) * scale,
            self.three_of_a_kind,
            (self.three_of_a_kind as f64) * scale,
            self.four_of_a_kind,
            (self.four_of_a_kind as f64) * scale,
            self.straight,
            (self.straight as f64) * scale,
            self.full_house,
            (self.full_house as f64) * scale,
            self.straight_flush,
            (self.straight_flush as f64) * scale
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::aggregate_score::AggregateScore;

    #[test]
    fn hello_world() {
        let scores = AggregateScore::default();
        println!("{scores}")
    }
}
