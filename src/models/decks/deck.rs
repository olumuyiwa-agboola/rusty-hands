use strum::IntoEnumIterator;
use rand::prelude::IndexedRandom;
use crate::models::cards::card::Card;
use crate::models::enums::card_rank::CardRank;
use crate::models::enums::card_suit::CardSuit;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for suit in CardSuit::iter() {
            for rank in CardRank::iter() {
                cards.push(Card::from(rank, suit));
            }
        }

        Deck { cards }
    }

    pub fn deal_cards(&mut self, number : usize) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();

        for _ in 1..=number {
            let mut rng = rand::rng();
            let random_card = self.cards.choose(&mut rng);

            match random_card {
                Some(card) => {
                    if let Some(index) = self.cards.iter().position(|x| x.suit == card.suit && x.rank == card.rank) {
                        let random_card_from_deck = self.cards.remove(index);
                        cards.push(random_card_from_deck);
                    };
                },
                None => {}
            }
        }

        cards
    }
}