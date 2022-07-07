use bevy::prelude::*;
use iyes_loopless::prelude::*;
use crate::core::{GameState, GridPosition};
use crate::core::Direction;
use crate::game_board::board;
use crate::game_board::helpers::move_grid_position;
use crate::food;
use crate::snake::head::SnakeHead;
use crate::snake::tail::SnakeTail;

use super::head;
use super::tail;

#[derive(Component)]
pub struct MovementController {
    pub direction: Direction,
    pub previous_position: GridPosition
}

pub fn handle_input(
    game_board: Res<board::Desc>,
    mut direction_events: EventReader<Direction>,
    mut query: Query<(&GridPosition,&mut MovementController), With<head::SnakeHead>>
){
    if !direction_events.is_empty() {
        if let Ok((grid_pos, mut controller)) = query.get_single_mut() {
            let new_direction = direction_events.iter().last().unwrap().clone();
            let predicted_position = move_grid_position(
                *grid_pos,
                new_direction.clone(),
                game_board.grid_size
            );
            // prevent snake reversing on itself immediately
            if predicted_position != controller.previous_position {
                controller.direction = new_direction;
            }
        }
    }
}

pub fn move_head(
    game_board: Res<board::Desc>,
    mut query: Query<(&mut GridPosition, &mut MovementController, With<head::SnakeHead>)>
){
    let (mut grid_pos, mut movement, _) = query.single_mut();
    movement.previous_position = *grid_pos;

    let updated_position = move_grid_position(
        *grid_pos,
        movement.direction.clone(),
        game_board.grid_size
    );
    *grid_pos = updated_position;
}


pub fn check_collide_with_food(
    mut head_query: Query<(&Transform, With<head::SnakeHead>)>,
    other_query: Query<((&Transform, Entity), With<food::FoodComponent>)>,
    mut consume_events: EventWriter<food::ConsumeEvent>,
) {
    if let Ok((head_transform, _)) = head_query.get_single_mut() {
        for ((transform, other_entity), _ ) in other_query.iter() {
            if head_transform.translation == transform.translation {
                consume_events.send(food::ConsumeEvent{target: other_entity});
                return;
            }
        }
    }
}

pub fn consume_food(
    game_board: Res<board::Desc>,
    mut commands: Commands,
    tail_query: Query<(Entity, &GridPosition, &SnakeTail)>,
    mut consume_events: EventReader<food::ConsumeEvent>,
) {
    if consume_events.iter().next().is_some() {
        println!("snake consume event");

        if let Some(end_of_tail) = tail_query
            .iter()
            .max_by(| a, b|{ a.2.index.cmp(&b.2.index) }) {
                let next_index = end_of_tail.2.index + 1;
                let follow_target_entity = end_of_tail.0;
                let follow_target_pos = *end_of_tail.1;

                tail::spawn_node(
                    &mut commands,
                    next_index,
                    game_board.cell_size as f32,
                    (follow_target_entity, follow_target_pos)
                );
        }
    }
}

pub fn check_for_bite_self(
    mut commands: Commands,
    head_query: Query<&GridPosition, With<SnakeHead>>,
    tail_query: Query<&GridPosition, With<SnakeTail>>
) {
    if let Ok(head_grid_pos) = head_query.get_single() {
        if tail_query
            .iter()
            .any(|tail_grid_pos| { head_grid_pos == tail_grid_pos }) {
            println!("dead at {:?}", head_grid_pos);
            commands.insert_resource(NextState(GameState::DEAD));
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use super::*;

    #[test]
    fn handle_input_basic() {
        let mut app = App::default();
        app.add_event::<Direction>();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        app.world
            .spawn()
            .insert(GridPosition::new(1, 0))
            .insert(MovementController{
                direction: Direction::Right,
                previous_position: GridPosition::new(-1, 0),
            })
            .insert(SnakeHead{});
        app.add_system(handle_input);

        app.world.resource_mut::<Events<Direction>>().send(Direction::Down);
        app.update();
        let movement_controller = app.world
            .query::<&MovementController>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(movement_controller.direction, Direction::Down);

        app.world.resource_mut::<Events<Direction>>().send(Direction::Right);
        app.update();
        let movement_controller = app.world
            .query::<&MovementController>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(movement_controller.direction, Direction::Right);
    }

    #[test]
    fn handle_input_prevent_reverse() {
        let mut app = App::default();
        app.add_event::<Direction>();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        app.world
            .spawn()
            .insert(GridPosition::new(1, 1))
            .insert(MovementController{
                direction: Direction::Right,
                previous_position: GridPosition::new(0, 1),
            })
            .insert(SnakeHead{});
        app.add_system(handle_input);

        app.world.resource_mut::<Events<Direction>>().send(Direction::Left);
        app.update();
        let movement_controller = app.world
            .query::<&MovementController>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(movement_controller.direction, Direction::Right);

        app.world.resource_mut::<Events<Direction>>().send(Direction::Down);
        app.update();
        let mut movement_controller = app.world
            .query::<&mut MovementController>()
            .iter_mut(&mut app.world)
            .next()
            .unwrap();
        assert_eq!(movement_controller.direction, Direction::Down);

        movement_controller.previous_position.set(&GridPosition::new(1, 0));
        app.world.resource_mut::<Events<Direction>>().send(Direction::Up);
        app.update();
        let movement_controller = app.world
            .query::<&MovementController>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(movement_controller.direction, Direction::Down);
    }
}