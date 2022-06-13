use bevy::prelude::*;
use super::helpers;
use super::controller::MovementController;
use crate::core::{GridPosition, Direction};
use crate::game_board::{GameBoardDesc, GameBoardHelpers};

#[derive(Component)]
pub struct SnakeHead {}

pub fn snake_head_sprite_position(
    game_board: Res<GameBoardDesc>,
    mut query: Query<(&GridPosition, &mut Transform), With<SnakeHead>>
) {
    if let Ok((grid_pos, mut transform)) = query.get_single_mut() {
        transform.translation = game_board.grid_pos_to_world_pos(grid_pos)
    }
}

pub fn spawn(commands: &mut Commands, cell_size: f32) {
    commands
        .spawn()
        .insert(SnakeHead{})
        .insert(GridPosition {
            x: -1, //init_data.start_position.x,
            y: -1 //init_data.start_position.y
        })
        .insert(MovementController{direction: Direction::Right})
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size));
}




