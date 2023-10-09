#[allow(dead_code)]

use std::sync::{Arc, Mutex};
mod player;
use boule::bubulle::*;
use crate::bubulle::Bubulle;
mod boule;

fn main() {
    println!("Hello, world!");

    let number_point1 = Arc::new(Mutex::new(100));
    let bubulle: Bubulle = Bubulle::new(5.0, "Bulle 1",1,  "Black", Arc::clone(&number_point1));

    let number_point2 = Arc::new(Mutex::new(500));
    let _bubulle2: Bubulle = Bubulle::new(9.0, "Bulle 2",5,  "Green", number_point2);
    
     let player1 = player::players::player::Player::new("Joueur1", "Host");
     let player2 = player::players::player::Player::new("Joueur2", "Participant");
     player1.display_information();
     player2.display_information();
     
     loop{
      

      bubulle.display_information();
      _bubulle2.display_information();

      let current_point1 = *number_point1.lock().unwrap();
        if current_point1 > 110 {
            bubulle.reduce_point(10);
        }

      // println!("name: {}, point : {}", bubulle2.membership, bubulle2.number_point);
      std::thread::sleep(std::time::Duration::from_secs(1));

     }
}
