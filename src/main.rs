use crate::game::Game;

mod card;
mod deck;
mod game;
mod player;

fn main() {
    let mut game = Game::new(2, 52);
    game.deal_initial_cards();
    game.player_hit(0);
    game.player_hit(1);
    game.dealer_play();

    println!(
        "Dealer outcome: {} {:?}",
        game.get_dealer_hand_value(),
        game.get_dealer_hand(),
    );

    println!(
        "Player 1 outcome: {:?} {} {:?}",
        game.get_outcome(0),
        game.get_player_hand_value(0),
        game.get_player_hand(0),
    );

    println!(
        "Player 2 outcome: {:?} {} {:?}",
        game.get_outcome(1),
        game.get_player_hand_value(1),
        game.get_player_hand(1),
    );
}
