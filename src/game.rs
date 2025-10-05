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
            player.hand.push(self.deck.deal());
            player.hand.push(self.deck.deal());
        }
        self.dealer.hand.push(self.deck.deal());
        self.dealer.hand.push(self.deck.deal());
    }

    pub fn player_hit(&mut self, player_index: usize) {
        if player_index >= self.players.len() {
            panic!("Invalid player index");
        }
        let player = &mut self.players[player_index];
        let card = self.deck.deal();
        player.hand.push(card);
    }

    pub fn dealer_play(&mut self) {
        while self.dealer.hand_value() < 17 {
            let card = self.deck.deal();
            self.dealer.hand.push(card);
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
