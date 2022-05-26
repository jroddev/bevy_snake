use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::core::GridPosition;

#[derive(Clone)]
pub struct GameBoardDesc {
    pub grid_size: (i32, i32),
    pub cell_size: i32,
}
pub trait GameBoardHelpers {
    fn grid_pos_to_world_pos(&self, grid_pos: &GridPosition) -> Vec3;
    fn world_pos_to_grid_pos(&self, translation: &Vec3) -> GridPosition;
}
pub struct GameBoardPlugin {
    pub desc: GameBoardDesc
}

impl Plugin for GameBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ShapePlugin)
            .add_startup_system(add_camera)
            .insert_resource(self.desc.clone())
            .add_startup_system(draw_origin);
    }
}

impl GameBoardHelpers for GameBoardDesc {
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

fn add_camera(game_board: Res<GameBoardDesc>, mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = (game_board.cell_size * game_board.grid_size.0) as f32 * 0.5;
    camera.transform.translation.y = -((game_board.cell_size * game_board.grid_size.1) as f32 * 0.5);
    commands.spawn_bundle(camera);

}

fn draw_origin(game_board: Res<GameBoardDesc>, mut commands: Commands) {
    let width = (game_board.grid_size.0 * game_board.cell_size) as f32;
    for i in 0..game_board.grid_size.0 + 1 {
        let horizontal = shapes::Line(
            Vec2::new(0., -(i * game_board.cell_size) as f32),
            Vec2::new(width, -(i * game_board.cell_size) as f32)
        );
        commands.spawn_bundle(GeometryBuilder::build_as(
            &horizontal,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ));
    }

    for i in 0..game_board.grid_size.1 + 1 {
        let vertical = shapes::Line(
            Vec2::new((i * game_board.cell_size) as f32, 0.),
            Vec2::new((i * game_board.cell_size) as f32, -width)
        );
        commands.spawn_bundle(GeometryBuilder::build_as(
            &vertical,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ));
    }
}