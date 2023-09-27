#[allow(dead_code)]

mod player;
use boule::bubulle::*;
use crate::bubulle::Bubulle;
mod boule;

fn main() {
    println!("Hello, world!");

    let bubulle: Bubulle = Bubulle::new(5.0, "Bulle 1",1,  "Black", 100);

    let bubulle2: Bubulle = Bubulle::new(9.0, "Bulle 2",5,  "Green", 500);
    let mut old_point : i32= 100;
    
     let player1 = player::players::player::Player::new("Joueur1", "Host");
     let player2 = player::players::player::Player::new("Joueur2", "Participant");
     player1.display_information();
     player2.display_information();
     
     loop{
        

        if bubulle.number_point != old_point {
        println!("name: {}, point : {}", bubulle.membership, bubulle.number_point);
        old_point = bubulle.number_point;
        }

        // println!("name: {}, point : {}", bubulle2.membership, bubulle2.number_point);
        

     }
}
