use crate::card::{Card, CardValue};

#[derive(Debug, PartialEq)]
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
        let mut hand_value = 0;
        let mut aces = 0;
        for card in hand {
            if let CardValue::Ace = card.value {
                aces += 1;
            }
            hand_value += card.get_card_value();
        }
        while hand_value > 21 && aces > 0 {
            hand_value -= 10;
            aces -= 1;
        }
        hand_value
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

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
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

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }
}

#[cfg(test)]
mod test {
    use crate::card::{Card, CardValue, Suit};

    fn card(value: CardValue) -> Card {
        Card::new(value, Suit::get_random_suit())
    }

    #[test]
    fn is_bust_when_player_over_21() {
        let player = super::Player {
            hand: vec![
                card(CardValue::Number(10)),
                card(CardValue::Number(5)),
                card(CardValue::Number(7)),
            ],
        };
        assert!(player.is_bust());
    }

    #[test]
    fn is_not_bust_when_player_21_or_under() {
        let player = super::Player {
            hand: vec![card(CardValue::Number(10)), card(CardValue::Number(5))],
        };
        assert!(!player.is_bust());
    }

    #[test]
    fn is_blackjack_when_player_21_with_two_cards() {
        let player = super::Player {
            hand: vec![card(CardValue::Ace), card(CardValue::King)],
        };
        assert!(player.is_blackjack());
    }

    #[test]
    fn is_not_blackjack_when_player_21_with_more_than_two_cards() {
        let player = super::Player {
            hand: vec![
                card(CardValue::Number(7)),
                card(CardValue::Number(7)),
                card(CardValue::Number(7)),
            ],
        };
        assert!(!player.is_blackjack());
    }

    #[test]
    fn player_wins_when_hand_greater_than_dealer() {
        let player = super::Player {
            hand: vec![card(CardValue::Number(10)), card(CardValue::Number(8))],
        };
        let dealer_value = 17;
        assert_eq!(player.get_outcome(dealer_value), super::PlayerOutcome::Win);
    }

    #[test]
    fn player_loses_when_hand_less_than_dealer() {
        let player = super::Player {
            hand: vec![card(CardValue::Number(10)), card(CardValue::Number(6))],
        };
        let dealer_value = 17;
        assert_eq!(player.get_outcome(dealer_value), super::PlayerOutcome::Lose);
    }

    #[test]
    fn player_pushes_when_hand_equals_dealer() {
        let player = super::Player {
            hand: vec![card(CardValue::Number(10)), card(CardValue::Number(7))],
        };
        let dealer_value = 17;
        assert_eq!(player.get_outcome(dealer_value), super::PlayerOutcome::Push);
    }

    #[test]
    fn player_outcome_is_bust_when_hand_over_21() {
        let player = super::Player {
            hand: vec![
                card(CardValue::Number(10)),
                card(CardValue::Number(5)),
                card(CardValue::Number(7)),
            ],
        };
        let dealer_value = 17;
        assert_eq!(player.get_outcome(dealer_value), super::PlayerOutcome::Bust);
    }

    #[test]
    fn player_outcome_is_blackjack_when_hand_is_blackjack() {
        let player = super::Player {
            hand: vec![card(CardValue::Ace), card(CardValue::King)],
        };
        let dealer_value = 20;
        assert_eq!(
            player.get_outcome(dealer_value),
            super::PlayerOutcome::Blackjack
        );
    }

    #[test]
    fn ace_value_changes_to_one_when_busting() {
        let player = super::Player {
            hand: vec![
                card(CardValue::Ace),
                card(CardValue::Number(9)),
                card(CardValue::Number(5)),
            ],
        };
        assert_eq!(player.hand_value(), 15);
    }

    #[test]
    fn multiple_aces_value_changes_to_one_when_busting() {
        let player = super::Player {
            hand: vec![
                card(CardValue::Ace),
                card(CardValue::Ace),
                card(CardValue::Number(10)),
            ],
        };
        assert_eq!(player.hand_value(), 12);
    }
}
