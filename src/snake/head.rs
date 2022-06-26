use bevy::prelude::*;
use super::helpers;
use super::controller::MovementController;
use crate::core::GridPosition;
use crate::core::Direction;
use crate::game_board::board;

#[derive(Component)]
pub struct SnakeHead{}

pub fn tick_position(
    game_board: Res<board::Desc>,
    mut query: Query<(&GridPosition, &mut Transform), With<SnakeHead>>
) {
    if let Ok((grid_pos, mut transform)) = query.get_single_mut() {
        transform.translation = game_board.grid_pos_to_world_pos(grid_pos)
    }
}

pub fn spawn(
    commands: &mut Commands,
    start_position: GridPosition,
    cell_size: f32
) -> Entity {
    commands
        .spawn()
        .insert(SnakeHead{})
        .insert(start_position)
        .insert(MovementController{
            direction: Direction::Right,
            previous_position: start_position
        })
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size))
        .id()
}


#[cfg(test)]
mod tests {
    use bevy::math::vec3;
    use rand::random;
    use super::*;

    #[derive(Clone, Copy)]
    struct HeadParams {
        start_position: GridPosition,
        cell_size: f32,
    }

    fn test_system(
        head_params: Res<HeadParams>,
        mut commands: Commands) {

        spawn(
            &mut commands,
            head_params.start_position,
            head_params.cell_size
        );
    }


    #[test]
    fn spawn_creates_head_entity() {
        let mut app = App::default();
        let head_params = HeadParams {
            start_position: GridPosition{x:3, y:3},
            cell_size: random::<f32>()
        };
        app.insert_resource(head_params);
        app.add_startup_system(test_system);
        app.update();
        let mut head_query = app.world.query::<(
            &SnakeHead,
            &GridPosition,
            &MovementController,
            With<Sprite>)>();
        assert_eq!(head_query.iter(&app.world).count(), 1);
        let (_, grid_pos, movement_controller, _) = head_query.iter(&app.world).next().unwrap();
        assert_eq!(grid_pos, &head_params.start_position);
        assert_eq!(&movement_controller.previous_position, &head_params.start_position);
        assert_eq!(movement_controller.direction, Direction::Right);
    }

    #[test]
    fn tick_position_sync_grid_pos_to_transform() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        let head_params = HeadParams {
            start_position: GridPosition{x:3, y:3},
            cell_size: random::<f32>()
        };
        app.insert_resource(head_params);
        app.add_startup_system(test_system);
        app.add_system(tick_position);
        app.update();

        let frames = vec![
            (GridPosition{x:0, y: 0}, vec3(0., 0., 0.)),
            (GridPosition{x:1, y: 0}, vec3(10., 0., 0.)),
            (GridPosition{x:2, y: 0}, vec3(20., 0., 0.)),
            (GridPosition{x:2, y: 1}, vec3(20., -10., 0.)),
            (GridPosition{x:2, y: 2}, vec3(20., -20., 0.)),
        ];
        for (frame_grid_pos, frame_translation, ) in frames {
            let (mut grid_pos, _) = app.world
                .query::<(&mut GridPosition, With<SnakeHead>)>()
                .iter_mut(&mut app.world)
                .next()
                .unwrap();
            grid_pos.set(&frame_grid_pos);

            app.update();

            let (transform, _) = app.world
                .query::<(&Transform, With<SnakeHead>)>()
                .iter_mut(&mut app.world)
                .next()
                .unwrap();
            assert_eq!(transform.translation, frame_translation);
        }
    }
}


