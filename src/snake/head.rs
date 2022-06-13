use bevy::prelude::*;
use super::helpers;
use super::controller::MovementController;
use crate::core::{GridPosition, Direction};
use crate::game_board::helpers::GameBoardHelpers;
use crate::game_board::board;

#[derive(Component)]
pub struct SnakeHead {}

pub fn snake_head_sprite_position(
    game_board: Res<board::Desc>,
    mut query: Query<(&GridPosition, &mut Transform), With<SnakeHead>>
) {
    if let Ok((grid_pos, mut transform)) = query.get_single_mut() {
        transform.translation = game_board.grid_pos_to_world_pos(grid_pos)
    }
}

pub fn spawn(commands: &mut Commands, start_position: GridPosition, cell_size: f32) {
    commands
        .spawn()
        .insert(SnakeHead{})
        .insert(start_position)
        .insert(MovementController{direction: Direction::Right})
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size));
}




