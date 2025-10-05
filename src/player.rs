use crate::card::{Card, CardValue};

#[derive(Debug)]
pub enum PlayerOutcome {
    Win,
    Lose,
    Bust,
    Blackjack,
    Push,
}

#[derive(Debug, Default)]
pub struct Player {
    pub hand: Vec<Card>,
}

#[derive(Debug, Default)]
pub struct Dealer {
    pub hand: Vec<Card>,
}

trait HandHolder {
    fn calculate_hand_value(&self, hand: &Vec<Card>) -> u8 {
        let mut value = 0;
        let mut aces = 0;
        for card in hand {
            match card.value {
                CardValue::Number(n) => value += n,
                CardValue::Jack | CardValue::Queen | CardValue::King => value += 10,
                CardValue::Ace => {
                    value += 11;
                    aces += 1;
                }
            }
        }
        while value > 21 && aces > 0 {
            value -= 10;
            aces -= 1;
        }
        value
    }
}

impl HandHolder for Player {}

impl Player {
    pub fn hand_value(&self) -> u8 {
        self.calculate_hand_value(&self.hand)
    }

    pub fn get_outcome(&self, dealer_value: u8) -> PlayerOutcome {
        let player_value = self.hand_value();
        if self.is_bust() {
            PlayerOutcome::Bust
        } else if dealer_value == player_value {
            PlayerOutcome::Push
        } else if self.is_blackjack() {
            PlayerOutcome::Blackjack
        } else if dealer_value > 21 || player_value > dealer_value {
            PlayerOutcome::Win
        } else {
            PlayerOutcome::Lose
        }
    }

    fn is_bust(&self) -> bool {
        self.hand_value() > 21
    }

    fn is_blackjack(&self) -> bool {
        self.hand_value() == 21 && self.hand.len() == 2
    }
}

impl HandHolder for Dealer {}

impl Dealer {
    pub fn hand_value(&self) -> u8 {
        self.calculate_hand_value(&self.hand)
    }
}
