use bevy::prelude::*;
use std::collections::VecDeque;
use crate::core::GridPosition;
use crate::game_board::board;
use super::helpers;

#[derive(Component)]
pub struct SnakeTail;

pub fn snake_tail_sprite_positions(
    game_board: Res<board::Desc>,
    position_history: Res<VecDeque<GridPosition>>,
    mut query: Query<&mut Transform, With<SnakeTail>>
) {
    for (index, mut transform) in query.iter_mut().enumerate() {
        if let Some(grid_pos) = &position_history.get(index) {
            transform.translation = game_board.grid_pos_to_world_pos(grid_pos);
        }
    }
}

pub fn spawn_node(commands: &mut Commands, cell_size: f32) {
    commands
        .spawn()
        .insert(SnakeTail{})
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size));
}