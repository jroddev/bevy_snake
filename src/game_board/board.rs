use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::GridPosition;

#[derive(Clone)]
pub struct Desc {
    pub grid_size: (i32, i32),
    pub cell_size: i32,
}

impl Desc {
    pub fn grid_pos_to_world_pos(&self, grid_pos: &GridPosition) -> Vec3 {
        Vec3::new(
            (grid_pos.x * self.cell_size) as f32,
            -(grid_pos.y * self.cell_size) as f32,
            0.
        )
    }
    pub fn world_pos_to_grid_pos(&self, translation: &Vec3) -> GridPosition {
        GridPosition {
            x: (translation.x.abs() as i32) / self.cell_size,
            y: (translation.y.abs() as i32) / self.cell_size,
        }
    }

    pub fn world_dimensions(&self,) -> (f32, f32) {
        let width = self.grid_size.0 * self.cell_size;
        let height = self.grid_size.1 * self.cell_size;
        (width as f32, height as f32)
    }
}

fn set_camera_pos(camera_transform: &mut Transform, game_board: &Desc) {
    camera_transform.translation.x = (game_board.cell_size * game_board.grid_size.0) as f32 * 0.5;
    camera_transform.translation.y = -((game_board.cell_size * game_board.grid_size.1) as f32 * 0.5);
}

pub fn spawn_camera(game_board: Res<Desc>, mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    set_camera_pos(&mut camera.transform, game_board.into_inner());
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

#[cfg(test)]
mod tests {
    use bevy::render::camera::Camera2d;
    use super::*;

    #[test]
    fn world_dimensions() {
        assert_eq!(Desc{ grid_size: (5, 5), cell_size: 10 }.world_dimensions(), (50.0, 50.0));
        assert_eq!(Desc{ grid_size: (15, 8), cell_size: 18 }.world_dimensions(), (270.0, 144.0));
    }

    #[test]
    fn grid_pos_to_world_pos() {
        let board = Desc{
            grid_size: (5, 5),
            cell_size: 10
        };

        assert_eq!(board.grid_pos_to_world_pos(&GridPosition{x: 5, y: 2}), Vec3::new(50., -20., 0.));
        assert_eq!(board.grid_pos_to_world_pos(&GridPosition{x: 7, y: -2}), Vec3::new(70., 20., 0.));

        let board = Desc{
            grid_size: (5, 5),
            cell_size: 8
        };

        assert_eq!(board.grid_pos_to_world_pos(&GridPosition{x: 5, y: 2}), Vec3::new(40., -16., 0.));
        assert_eq!(board.grid_pos_to_world_pos(&GridPosition{x: 7, y: -2}), Vec3::new(56., 16., 0.));
    }

    #[test]
    fn world_pos_to_grid_pos() {
        let board = Desc {
            grid_size: (5, 5),
            cell_size: 10
        };

        assert_eq!(board.world_pos_to_grid_pos(&Vec3::new(50., -20., 0.)), GridPosition{x: 5, y: 2});
        assert_eq!(board.world_pos_to_grid_pos(&Vec3::new(70., 20., 0.)), GridPosition{x: 7, y: 2});

        let board = Desc{
            grid_size: (5, 5),
            cell_size: 8
        };

        assert_eq!(board.world_pos_to_grid_pos(&Vec3::new(40., -16., 0.)), GridPosition{x: 5, y: 2});
        assert_eq!(board.world_pos_to_grid_pos(&Vec3::new(56., 16., 0.)), GridPosition{x: 7, y: 2});
    }

    #[test]
    fn spawn_camera() {
        let mut app = App::default();
        let board = Desc {
            grid_size: (5, 5),
            cell_size: 10
        };
        app.world.insert_resource(board);
        app.add_system(super::spawn_camera);
        app.update();
        app.world
            .query::<(&Transform, With<Camera2d>)>()
            .iter(&app.world)
            .map(|(t, _)|t)
            .next()
            .unwrap();
    }

    #[test]
    fn set_camera_pos() {
        let mut camera_transform = Transform::default();
        let board = Desc {
            grid_size: (5, 5),
            cell_size: 10
        };
        super::set_camera_pos(&mut camera_transform, &board);
        assert_eq!(
            camera_transform,
            Transform::from_xyz(25., -25., 0.)
        )
    }
}
