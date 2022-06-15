pub mod core;
mod game_board;
mod input;
mod snake;
mod food;
mod direction;

use std::time::Duration;
use bevy::prelude::*;
use crate::core::{GamePlugin, GameWindow, GridPosition};
use crate::food::FoodPlugin;
use crate::game_board::board;
use crate::game_board::plugin::GameBoardPlugin;
use crate::input::GameInputPlugin;

const TICK_TIME_SECONDS: f32 = 0.1;
const GRID_SIZE: (i32, i32) = (15, 15);
const CELL_SIZE: i32 = 15;
const SNAKE_START_POS: GridPosition = GridPosition{x: 0, y: 7};
const FOOD_START_POS: GridPosition = GridPosition{x: 7, y: 7};
const START_TAIL_LENGTH: usize = 3;

fn main() {
    println!("Hello, Snake!");
    let game_board_desc = board::Desc{
        grid_size: GRID_SIZE,
        cell_size: CELL_SIZE,
    };
    let window_desc = GameWindow {
        title: "Bevy Snake".to_string(),
        width: (game_board_desc.grid_size.0 * game_board_desc.cell_size) as f32,
        height: (game_board_desc.grid_size.1 * game_board_desc.cell_size) as f32,
    };

    App::new()
        .add_plugin(GamePlugin{
            window: window_desc,
            tick_time_seconds: TICK_TIME_SECONDS
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameBoardPlugin { desc: game_board_desc })
        .add_plugin(GameInputPlugin)
        .add_plugin(FoodPlugin {
            init_params: food::InitParams {
                start_position: FOOD_START_POS
            }
        })
        .add_plugin(snake::plugin::SnakePlugin {
            init_params: snake::helpers::InitParams{
                movement_time_step: Duration::from_secs_f32(TICK_TIME_SECONDS),
                start_position: SNAKE_START_POS,
                initial_tail_length: START_TAIL_LENGTH
            }
        })
        .run();
}












