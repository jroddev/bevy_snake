use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Clone)]
pub struct Desc {
    pub grid_size: (i32, i32),
    pub cell_size: i32,
}

pub fn add_camera(game_board: Res<Desc>, mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = (game_board.cell_size * game_board.grid_size.0) as f32 * 0.5;
    camera.transform.translation.y = -((game_board.cell_size * game_board.grid_size.1) as f32 * 0.5);
    commands.spawn_bundle(camera);
}

pub fn draw_origin(game_board: Res<Desc>, mut commands: Commands) {
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