use rand::Rng;

#[derive(Debug)]
pub enum CardValue {
    King,
    Queen,
    Jack,
    Ace,
    Number(u8),
}

#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Card {
    pub value: CardValue,
    suit: Suit,
}

impl Card {
    fn new(value: CardValue, suit: Suit) -> Self {
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
        let suit = match rng.random_range(0..4) {
            0 => Suit::Hearts,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            _ => Suit::Spades,
        };
        Card { value, suit }
    }
}
