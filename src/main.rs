use macroquad::prelude::*;
use std::time::Duration;

mod board;
mod game;
mod move_generator;
mod moves;
mod piece;
mod render;
mod utils;
mod bot;

use game::{Game, HumanPlayer};

use crate::bot::Bot;

#[macroquad::main("Chess")]
async fn main() {
    let mut game = Game::new().await;

    game.add_player(HumanPlayer::new("Tim"))
        .add_player(Bot::new("Bot"))
        .set_time(Duration::from_secs(5 * 60), Duration::from_secs(3))
        .set_flip_board_each_move(false);

    while game.update() {
        next_frame().await;
    }
}