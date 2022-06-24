use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::collections::VecDeque;
use crate::core::{GameState, GridPosition};
use crate::direction::Direction;
use crate::game_board::board;
use crate::food;

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
                grid_pos.clone(),
                new_direction.clone(),
                game_board.grid_size
            );
            if predicted_position != controller.previous_position {
                controller.direction = new_direction;
            }
        }
    }
}

fn move_grid_position(
    mut grid_pos: GridPosition,
    direction: Direction,
    grid_size: (i32, i32)) -> GridPosition {

    match direction {
        Direction::Up => grid_pos.y -= 1,
        Direction::Down => grid_pos.y += 1,
        Direction::Left => grid_pos.x -= 1,
        Direction::Right => grid_pos.x += 1
    }

    // Wrap Around
    grid_pos.x = (grid_pos.x + grid_size.0) % grid_size.0;
    grid_pos.y = (grid_pos.y + grid_size.1) % grid_size.1;
    grid_pos
}

pub fn move_head(
    game_board: Res<board::Desc>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut query: Query<(&mut GridPosition, &mut MovementController, With<head::SnakeHead>)>
){
    let (mut grid_pos, mut movement, _) = query.single_mut();
    movement.previous_position = grid_pos.clone();
    position_history.pop_front();
    position_history.push_back(grid_pos.clone());

    let updated_position = move_grid_position(
        grid_pos.clone(),
        movement.direction.clone(),
        game_board.grid_size
    );
    grid_pos.x = updated_position.x;
    grid_pos.y = updated_position.y;
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
    query: Query<(&GridPosition, With<head::SnakeHead>)>,
    mut consume_events: EventReader<food::ConsumeEvent>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut commands: Commands
) {
    if consume_events.iter().next().is_some() {
        println!("snake consume event");
        let (grid_pos, _) = query.single();
        position_history.push_back(grid_pos.clone());
        tail::spawn_node(&mut commands, game_board.cell_size as f32);
    }
}

pub fn check_for_bite_self(
    mut commands: Commands,
    query: Query<&GridPosition, With<head::SnakeHead>>,
    position_history: ResMut<VecDeque<GridPosition>>,
) {
    if let Ok(head) = query.get_single() {
        position_history
            .iter()
            .take(position_history.len()-1)
            .for_each(|pos|{
                if pos == head {
                    println!("dead at {:?} with a score of {}", pos, position_history.len());
                    commands.insert_resource(NextState(GameState::DEAD));
                }
            })
    }
}

