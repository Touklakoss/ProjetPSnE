#![allow(dead_code)]

mod GUI;
mod game_core;

use GUI::GUI::*;
use game_core::game_core::*;

fn main() {
    let mut gameCore = Game_core::new();

    gameCore.game_core_run();
}
