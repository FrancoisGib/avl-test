use rand::seq::SliceRandom;

use crate::card::{Card, CardValue, Suit};

pub struct Deck {
    n: u16,
    pub cards: Vec<Card>,
}

const CARD_GAME_SIZE: usize = 52;

impl Deck {
    pub fn new(nb_cards: u16) -> Self {
        if nb_cards == 0 {
            panic!("Deck cannot be empty");
        }
        let mut cards = Vec::with_capacity(nb_cards as usize);
        for _ in 0..nb_cards {
            cards.push(Card::get_random_card());
        }
        Deck { n: nb_cards, cards }
    }

    pub fn from_one_card_game() -> Self {
        let mut cards = Vec::with_capacity(CARD_GAME_SIZE);
        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in 2..=10 {
                cards.push(Card::new(CardValue::Number(value), suit));
            }
            cards.push(Card::new(CardValue::Jack, suit));
            cards.push(Card::new(CardValue::Queen, suit));
            cards.push(Card::new(CardValue::King, suit));
            cards.push(Card::new(CardValue::Ace, suit));
        }
        Deck::from_vec(cards)
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::card::Card;

    #[test]
    fn test_deck_initialization_from_one_card_game_all_unique() {
        let deck = Deck::from_one_card_game();
        assert_eq!(deck.n, 52);
        assert_eq!(deck.cards.len(), 52);

        let unique_cards: HashSet<&Card> = deck.cards.iter().collect();
        assert_eq!(unique_cards.len(), 52);
    }

    #[test]
    fn test_deal_reduces_deck_size() {
        let mut deck = Deck::new(10);
        let initial_size = deck.cards.len();
        let _ = deck.deal();
        assert!(deck.cards.len() < initial_size);
    }

    #[test]
    fn test_deal_refill_deck_when_empty() {
        let mut deck = Deck::new(2);
        let _ = deck.deal();
        let _ = deck.deal();
        assert!(deck.cards.len() == 0);
        let _ = deck.deal();
        assert!(deck.cards.len() == 1);
    }

    #[test]
    #[should_panic(expected = "Deck cannot be empty")]
    fn test_deck_new_zero_cards_panics() {
        Deck::new(0);
    }

    #[test]
    #[should_panic(expected = "Deck cannot be empty")]
    fn test_deck_from_vec_empty_panics() {
        Deck::from_vec(vec![]);
    }

    // tests added for mutations
    #[test]
    fn test_shuffle_changes_card_order() {
        let mut deck = Deck::from_one_card_game();
        let original_order: Vec<Card> = deck.cards.clone();
        deck.shuffle();
        assert_ne!(deck.cards, original_order);
    }
}
