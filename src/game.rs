use crate::{
    card::Card,
    deck::Deck,
    player::{Dealer, Player, PlayerOutcome},
};

pub struct Game {
    pub players: Vec<Player>,
    pub dealer: Dealer,
    pub deck: Deck,
}

impl Game {
    pub fn new(nb_players: u8, nb_cards: u16) -> Self {
        if nb_players == 0 {
            panic!("There must be at least one player");
        }

        let mut players = Vec::new();
        for _ in 0..nb_players {
            players.push(Player::default());
        }

        let dealer = Dealer::default();
        let mut deck = Deck::new(nb_cards);
        deck.shuffle();

        Game {
            players,
            dealer,
            deck,
        }
    }

    pub fn deal_initial_cards(&mut self) {
        for player in &mut self.players {
            player.add_card(self.deck.deal());
            player.add_card(self.deck.deal());
        }
        self.dealer.add_card(self.deck.deal());
        self.dealer.add_card(self.deck.deal());
    }

    pub fn player_hit(&mut self, player_index: usize) {
        if player_index >= self.players.len() {
            panic!("Invalid player index");
        }
        let player = &mut self.players[player_index];
        let card = self.deck.deal();
        player.add_card(card);
    }

    pub fn dealer_play(&mut self) {
        while self.dealer.hand_value() < 17 {
            let card = self.deck.deal();
            self.dealer.add_card(card);
        }
    }

    pub fn get_outcome(&self, player_index: usize) -> PlayerOutcome {
        if player_index >= self.players.len() {
            panic!("Invalid player index");
        }
        self.players[player_index].get_outcome(self.dealer.hand_value())
    }

    pub fn get_player_hand(&self, player_index: usize) -> &Vec<Card> {
        if player_index >= self.players.len() {
            panic!("Invalid player index");
        }
        &self.players[player_index].hand
    }

    pub fn get_player_hand_value(&self, player_index: usize) -> u8 {
        if player_index >= self.players.len() {
            panic!("Invalid player index");
        }
        self.players[player_index].hand_value()
    }

    pub fn get_dealer_hand(&self) -> &Vec<Card> {
        &self.dealer.hand
    }

    pub fn get_dealer_hand_value(&self) -> u8 {
        self.dealer.hand_value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, CardValue, Suit};

    fn card(value: CardValue) -> Card {
        Card::new(value, Suit::get_random_suit())
    }

    #[test]
    fn test_game_initialization() {
        let game = Game::new(2, 10);
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.dealer.hand.len(), 0);
        assert!(game.deck.cards.len() == 10);
    }

    #[test]
    #[should_panic(expected = "There must be at least one player")]
    fn test_game_zero_players_panics() {
        Game::new(0, 10);
    }

    #[test]
    fn test_deal_initial_cards() {
        let mut game = Game::new(2, 20);
        game.deal_initial_cards();

        for player in &game.players {
            assert_eq!(player.hand.len(), 2);
        }
        assert_eq!(game.dealer.hand.len(), 2);
    }

    #[test]
    fn test_player_hit_adds_card() {
        let mut game = Game::new(1, 10);
        game.deal_initial_cards();

        let initial_hand_len = game.players[0].hand.len();
        game.player_hit(0);
        assert_eq!(game.players[0].hand.len(), initial_hand_len + 1);
    }

    #[test]
    #[should_panic(expected = "Invalid player index")]
    fn test_player_hit_invalid_index_panics() {
        let mut game = Game::new(1, 10);
        game.player_hit(5);
    }

    #[test]
    fn test_dealer_play_reaches_17_or_more() {
        let mut game = Game::new(1, 20);
        game.deal_initial_cards();

        game.dealer_play();
        assert!(game.dealer.hand_value() >= 17);
    }

    #[test]
    fn get_outcome_when_player_pushes() {
        let mut game = Game::new(1, 20);

        game.dealer.hand = vec![card(CardValue::Number(10)), card(CardValue::Number(7))];

        game.players[0].hand = vec![card(CardValue::Number(10)), card(CardValue::Number(7))];

        let outcome = game.get_outcome(0);
        assert_eq!(outcome, PlayerOutcome::Push);
    }

    #[test]
    fn get_outcome_when_player_busts() {
        let mut game = Game::new(1, 20);
        game.deal_initial_cards();

        game.players[0].hand = vec![
            card(CardValue::Number(10)),
            card(CardValue::Number(10)),
            card(CardValue::Number(10)),
        ];

        let outcome = game.get_outcome(0);
        assert_eq!(outcome, PlayerOutcome::Bust);
    }

    #[test]
    fn get_outcome_when_player_has_blackjack_and_dealer_lose() {
        let mut game = Game::new(1, 20);

        game.players[0].hand = vec![card(CardValue::Ace), card(CardValue::King)];
        game.dealer.hand = vec![card(CardValue::Number(10)), card(CardValue::Number(7))];

        let outcome = game.get_outcome(0);
        assert_eq!(outcome, PlayerOutcome::Blackjack);
    }

    #[test]
    fn get_outcome_when_player_has_blackjack_and_dealer_too() {
        let mut game = Game::new(1, 20);

        game.players[0].hand = vec![card(CardValue::Ace), card(CardValue::King)];
        game.dealer.hand = vec![card(CardValue::Ace), card(CardValue::King)];

        let outcome = game.get_outcome(0);
        assert_eq!(outcome, PlayerOutcome::Push);
    }

    #[test]
    #[should_panic(expected = "Invalid player index")]
    fn test_get_outcome_invalid_index_panics() {
        let game = Game::new(1, 10);
        game.get_outcome(5);
    }

    // modified for mutations
    #[test]
    fn test_get_player_hand_value() {
        let mut game = Game::new(1, 10);
        game.deal_initial_cards();

        let hand = game.get_player_hand(0);
        let value = game.get_player_hand_value(0);
        assert_eq!(hand.len(), 2);
        assert_eq!(value, game.players[0].hand_value());
    }

    // modified for mutations
    #[test]
    fn test_get_dealer_hand_value() {
        let mut game = Game::new(1, 10);
        game.deal_initial_cards();

        let hand = game.get_dealer_hand();
        let value = game.get_dealer_hand_value();
        assert_eq!(hand.len(), 2);
        assert_eq!(value, game.dealer.hand_value());
    }

    // tests added for mutations
    #[test]
    fn dealer_not_hit_at_17() {
        let mut game = Game::new(1, 10);

        game.dealer.hand = vec![card(CardValue::Number(10)), card(CardValue::Number(7))];
        let initial_hand_len = game.dealer.hand.len();

        game.dealer_play();
        assert_eq!(game.dealer.hand.len(), initial_hand_len);
    }

    #[test]
    fn dealer_play_until_17() {
        let mut game = Game::new(1, 10);
        assert_eq!(game.dealer.hand.len(), 0);
        game.dealer_play();
        assert!(game.dealer.hand_value() >= 17);
        assert!(game.dealer.hand.len() > 0);
    }
}
