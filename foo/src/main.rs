#![allow(dead_code)]

mod player;
mod bubulle;
mod displayer;

use displayer::displayer::*;

fn main() {
    println!("Hello, world!");

    let bubulle = bubulle::bubulle::Bubulle::new(5.0, 100, String::from("Bulle 1"), 1);
    bubulle.display_information();

    let player1 = player::player::Player::new("Joueur1", "Host");
    let player2 = player::player::Player::new("Joueur2", "Participant");
    player1.display_information();
    player2.display_information();

    let mut displayer = Displayer::new();
    displayer.render_window();
}
