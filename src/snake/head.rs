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
}


