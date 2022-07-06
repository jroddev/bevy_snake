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
    use bevy::ecs::system::SystemState;
    use bevy::math::vec3;
    use rand::random;
    use super::*;

    #[test]
    fn spawn_creates_head_entity() {
        let mut app = App::default();
        let start_position = GridPosition{x:3, y:3};
        let cell_size = random::<f32>();
        let mut state: SystemState<Commands> = SystemState::new(&mut (app.world));
        let mut commands = state.get_mut(&mut (app.world));
        let entity_id = spawn(
            &mut commands,
            start_position,
            cell_size
        );
        state.apply(&mut app.world);

        let mut head_query = app.world.query::<(
            &SnakeHead,
            &GridPosition,
            &MovementController,
            With<Sprite>)>();

        assert_eq!(head_query.iter(&app.world).count(), 1);
        let (_, grid_pos, movement_controller, _) = head_query.iter(&app.world).next().unwrap();
        assert_eq!(grid_pos, &start_position);
        assert_eq!(&movement_controller.previous_position, &start_position);
        assert_eq!(movement_controller.direction, Direction::Right);
    }

    #[test]
    fn tick_position_sync_grid_pos_to_transform() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        let start_position = GridPosition{x:3, y:3};
        let cell_size = random::<f32>();
        let mut state: SystemState<Commands> = SystemState::new(&mut (app.world));
        let mut commands = state.get_mut(&mut (app.world));
        let entity_id = spawn(
            &mut commands,
            start_position,
            cell_size
        );
        state.apply(&mut app.world);
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


