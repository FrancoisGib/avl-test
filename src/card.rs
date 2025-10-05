use rand::Rng;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CardValue {
    King,
    Queen,
    Jack,
    Ace,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn get_random_suit() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            _ => Suit::Spades,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[allow(unused)]
pub struct Card {
    pub value: CardValue,
    suit: Suit,
}

#[allow(dead_code)]
impl Card {
    pub fn new(value: CardValue, suit: Suit) -> Self {
        if let CardValue::Number(n) = value {
            if n < 2 || n > 10 {
                panic!("Card number must be between 2 and 10");
            }
        }
        Card { value, suit }
    }

    pub fn get_random_card() -> Self {
        let mut rng = rand::rng();
        let value = match rng.random_range(1..=13) {
            1 => CardValue::Ace,
            11 => CardValue::Jack,
            12 => CardValue::Queen,
            13 => CardValue::King,
            n => CardValue::Number(n as u8),
        };
        let suit = Suit::get_random_suit();
        Card { value, suit }
    }

    pub fn get_card_value(&self) -> u8 {
        match self.value {
            CardValue::Number(n) => n,
            CardValue::Jack | CardValue::Queen | CardValue::King => 10,
            CardValue::Ace => 11,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Card;
    use super::CardValue;
    use super::Suit;

    fn card(value: CardValue) -> Card {
        Card::new(value, Suit::get_random_suit())
    }

    #[test]
    #[should_panic(expected = "Card number must be between 2 and 10")]
    fn numbered_card_cannot_be_below_2_should_panic() {
        card(CardValue::Number(1));
    }

    #[test]
    #[should_panic(expected = "Card number must be between 2 and 10")]
    fn numbered_card_cannot_be_above_10_should_panic() {
        card(CardValue::Number(11));
    }

    #[test]
    fn normal_card_value_between_2_and_10() {
        for n in 2..=10 {
            let card = super::Card::new(CardValue::Number(n), super::Suit::Hearts);
            assert_eq!(card.get_card_value(), n);
        }
    }

    #[test]
    fn figure_card_value_is_10() {
        let jack = super::Card::new(CardValue::Jack, super::Suit::Hearts);
        let queen = super::Card::new(CardValue::Queen, super::Suit::Diamonds);
        let king = super::Card::new(CardValue::King, super::Suit::Clubs);
        assert_eq!(jack.get_card_value(), 10);
        assert_eq!(queen.get_card_value(), 10);
        assert_eq!(king.get_card_value(), 10);
    }

    #[test]
    fn ace_card_value_is_11() {
        let ace = super::Card::new(CardValue::Ace, super::Suit::Spades);
        assert_eq!(ace.get_card_value(), 11);
    }
}
