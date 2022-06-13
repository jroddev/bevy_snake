use bevy::prelude::*;
use crate::core::GridPosition;

use super::board;

// fn grid_pos_to_world_pos(game_board_desc: GameBoardDesc, grid_pos: &GridPosition) -> Vec3 {
//     Vec3::new(
//         (grid_pos.x * self.cell_size) as f32,
//         -(grid_pos.y * self.cell_size) as f32,
//         0.
//     )
// }


pub trait GameBoardHelpers {
    fn grid_pos_to_world_pos(&self, grid_pos: &GridPosition) -> Vec3;
    fn world_pos_to_grid_pos(&self, translation: &Vec3) -> GridPosition;
}

impl GameBoardHelpers for board::Desc {
    fn grid_pos_to_world_pos(&self, grid_pos: &GridPosition) -> Vec3 {
        Vec3::new(
            (grid_pos.x * self.cell_size) as f32,
            -(grid_pos.y * self.cell_size) as f32,
            0.
        )
    }
    fn world_pos_to_grid_pos(&self, translation: &Vec3) -> GridPosition {
        GridPosition {
            x: (translation.x.abs() as i32) / self.cell_size,
            y: (translation.y.abs() as i32) / self.cell_size,
        }
    }
}


#[test]
fn test_grid_pos_to_world_pos() {
    assert_eq!(1,2);
}