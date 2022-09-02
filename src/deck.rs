use crate::card::Card;
use crate::card::Hand;
use crate::card::Rank;
use crate::card::Suit;
use std::collections::HashMap;

/// Represent a standard deck of cards with all suits and ranks present.
/// The primary use of this data structure is to perform quick lookup for
/// cards by name, which is implemented by a HashMap behind the scenes.
pub struct Deck {
    cards_by_name: HashMap<String, Card>,
}

impl Deck {
    #[allow(dead_code)]
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards_by_name: HashMap::new(),
        };
        for rank_id in 0..Rank::NUM_RANKS {
            for suit_id in 0..Suit::NUM_SUITS {
                let rank = Rank { id: rank_id };
                let suit = Suit { id: suit_id };
                let card = Card::new(&rank, &suit);
                deck.cards_by_name.insert(card.to_string(), card);
            }
        }
        deck
    }

    #[allow(dead_code)]
    pub fn draw_card(&self, card_name: &String) -> Option<&Card> {
        self.cards_by_name.get(card_name)
    }

    #[allow(dead_code)]
    pub fn draw_hand<const N: usize>(&self, card_names: &[&str; N]) -> Option<Hand<N>> {
        let mut hand = Hand {
            cards: array_init::array_init(|_| Card { id: 0 }),
        };
        for i in 0..N {
            // TODO:  can we use the slice directly instead?
            if let Some(card) = self.draw_card(&String::from(card_names[i])) {
                hand.cards[i] = *card;
            } else {
                return None;
            }
        }
        Some(hand)
    }
}

//////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::deck::Deck;

    #[test]
    fn draw_card_by_name_test() {
        let deck = Deck::new();
        for card_name in vec!["A♦", "5♥", "Q♠", "2♣"]
            .into_iter()
            .map(|name_slice| String::from(name_slice))
        {
            assert_eq!(deck.draw_card(&card_name).unwrap().to_string(), card_name);
        }
    }
}
