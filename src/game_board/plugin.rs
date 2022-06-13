use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use super::board;

pub struct GameBoardPlugin {
    pub desc: board::Desc
}

impl Plugin for GameBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(board::add_camera)
            .insert_resource(self.desc.clone())
            .add_startup_system(board::draw_origin);
    }
}