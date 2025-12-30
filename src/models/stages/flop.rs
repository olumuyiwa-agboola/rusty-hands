use crate::models::cards::card::Card;
use crate::models::decks::deck::Deck;

#[derive(Debug)]
pub struct Flop {
    pub cards: [Card; 3]
}

impl Flop {
    pub fn from(deck: &mut Deck) -> Self {
        let cards = deck.deal_cards(3);

        Flop {
            cards: cards.try_into().unwrap()
        }
    }
}