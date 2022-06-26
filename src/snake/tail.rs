use bevy::prelude::*;
use crate::core::GridPosition;
use crate::game_board::board;
use super::helpers;

#[derive(Component)]
pub struct SnakeTail{
    pub index: usize,
    pub follow_target: Entity,
    pub next_position: GridPosition
}

pub fn tick_position(
    game_board: Res<board::Desc>,
    mut tail_query: Query<(Entity, &mut Transform, &mut SnakeTail)>,
    mut grid_pos_query: Query<&mut GridPosition>,
) {
    for (tail_segment, mut transform, mut tail) in tail_query.iter_mut() {
        if let Ok(target_grid_pos) = grid_pos_query.get(tail.follow_target) {
            if target_grid_pos != &tail.next_position {
                let new_next_position = target_grid_pos.clone();
                if let Ok(mut current_grid_pos) = grid_pos_query.get_mut(tail_segment) {
                    transform.translation = game_board.grid_pos_to_world_pos(&tail.next_position);
                    current_grid_pos.x = tail.next_position.x;
                    current_grid_pos.y = tail.next_position.y;
                    tail.next_position.x = new_next_position.x;
                    tail.next_position.y = new_next_position.y;
                }
            }
        }
    }
}

pub fn spawn_node(
    commands: &mut Commands,
    tail_index: usize,
    cell_size: f32,
    follow_target: (Entity, GridPosition)
) -> Entity {
    println!("spawn tail segment: {}", tail_index);
    commands
        .spawn()
        .insert(SnakeTail{
            index: tail_index,
            follow_target: follow_target.0,
            next_position: follow_target.1
        })
        .insert(GridPosition{ x: -1, y: -1})
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size))
        .id()
}