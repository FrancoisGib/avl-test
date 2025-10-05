use rand::seq::SliceRandom;

use crate::card::Card;

pub struct Deck {
    n: u16,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(nb_cards: u16) -> Self {
        if nb_cards == 0 {
            panic!("Deck cannot be empty");
        }
        let mut cards = Vec::new();
        for _ in 0..nb_cards {
            cards.push(Card::get_random_card());
        }
        Deck { n: nb_cards, cards }
    }

    pub fn from_vec(cards: Vec<Card>) -> Self {
        if cards.len() == 0 {
            panic!("Deck cannot be empty");
        }
        Deck {
            n: cards.len() as u16,
            cards,
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    pub fn deal(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            card
        } else {
            for _ in 0..self.n {
                self.cards.push(Card::get_random_card());
            }
            self.cards.pop().unwrap()
        }
    }
}
