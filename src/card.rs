use rand::Rng;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct Suit {
    pub id: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct Rank {
    pub id: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Copy, Clone, Hash)]
pub struct Card {
    pub id: usize,
}

//////////////////////////////////////////////////////////////////////////////////////
impl Suit {
    pub const NUM_SUITS: usize = 4;
}

/// One of:  {♥, ♣, ♦, ♠, ?}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.id {
                0 => String::from("♣"),
                1 => String::from("♦"),
                2 => String::from("♥"),
                3 => String::from("♠"),
                _ => String::from("?"),
            }
        )
    }
}

impl Rank {
    #[allow(dead_code)]
    pub const NUM_RANKS: usize = 13;
}

/// One of:  {A, 2, 3, ... 8, 9, T, J, Q, K}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let one_based_index = self.id + 1;

        write!(
            f,
            "{}",
            match one_based_index {
                1 => String::from("A"),
                10 => String::from("T"),
                11 => String::from("J"),
                12 => String::from("Q"),
                13 => String::from("K"),
                _ => one_based_index.to_string(),
            }
        )
    }
}

impl Card {
    pub const NUM_CARDS: usize = Suit::NUM_SUITS * Rank::NUM_RANKS;

    #[allow(dead_code)]
    pub fn new(rank: &Rank, suit: &Suit) -> Card {
        Card {
            id: rank.id * Suit::NUM_SUITS + suit.id,
        }
    }

    /// Returns the suit (club, spade, heart, diamond) of this [`Card`],
    /// where the suit is represented as an integer.
    pub fn suit(&self) -> Suit {
        Suit {
            id: (self.id % Suit::NUM_SUITS),
        }
    }

    /// Returns the "index" (ace, two, three, ... jack, queen, king) of the [`Card`].
    pub fn rank(&self) -> Rank {
        Rank {
            id: (self.id / Suit::NUM_SUITS),
        }
    }

    pub fn draw_random_card<R: Rng>(rng: &mut R) -> Card {
        Card {
            id: rng.gen_range(0..Card::NUM_CARDS),
        }
    }
}

/// Examples:  K♥ A♣ Q♥ 3♦ 7 8♠ 2♥
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank(), self.suit())
    }
}

//////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::card::Card;
    use crate::card::Rank;
    use crate::card::Suit;

    #[test]
    fn card_constructor_from_rank_and_suit_test() {
        for rank_id in 0..Rank::NUM_RANKS {
            for suit_id in 0..Suit::NUM_SUITS {
                let rank = Rank { id: rank_id };
                let suit = Suit { id: suit_id };
                let card = Card::new(&rank, &suit);
                assert_eq!(card.suit(), suit);
                assert_eq!(card.rank(), rank);
            }
        }
    }
}
