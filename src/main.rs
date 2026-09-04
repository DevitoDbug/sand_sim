use macroquad::prelude::*;

use crate::engine::{
    game::Game,
    global_const::{BLOCK_SIZE, GRID_COLUMNS, GRID_ROWS},
};

mod engine;

#[macroquad::main(get_game_conf())]
async fn main() {
    let mut game = Game::new();
    game.render().await;
}

fn get_game_conf() -> Conf {
    Conf {
        window_title: String::from("Sand game"),
        window_width: BLOCK_SIZE * GRID_COLUMNS,
        window_height: BLOCK_SIZE * GRID_ROWS,
        window_resizable: false,
        ..Default::default()
    }
}
