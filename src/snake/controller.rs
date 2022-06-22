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
    pub direction: Direction
}

pub fn handle_input(
    position_history: ResMut<VecDeque<GridPosition>>,
    mut direction_events: EventReader<Direction>,
    mut query: Query<(&GridPosition,&mut MovementController), With<head::SnakeHead>>
){
    if !direction_events.is_empty() {
        if let Ok((grid_pos, mut controller)) = query.get_single_mut() {
            let new_direction = direction_events.iter().last().unwrap().clone();

            // On a food consume frame we push an extra tail node onto the array
            // at the position of the head.
            // This conflicts with the logic to prevent reversing direction
            // If we detect this, select the next node down the chain
            let head_moved = &position_history[position_history.len() - 1] != grid_pos;
            let previous_position_index = if head_moved {
                position_history.len() - 1
            } else {
                position_history.len() - 2
            };
            let previous_position = &position_history[previous_position_index];

            if let Some(reverse_direction) = Direction::between(grid_pos, previous_position) {
                // Prevent reversing directly onto itself
                if new_direction != reverse_direction {
                    controller.direction = new_direction;
                }
            } else {
                eprintln!("grid pos: {:?}, pos history: {:?}", grid_pos, position_history);
                panic!("no direction between {:?} and {:?}", grid_pos, previous_position)
            }
        }
    }
}

pub fn move_head(
    game_board: Res<board::Desc>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut query: Query<(&mut GridPosition, &MovementController, With<head::SnakeHead>)>
){
    let (mut grid_pos, movement, _) = query.single_mut();
    position_history.pop_front();
    position_history.push_back(grid_pos.clone());

    match movement.direction {
        Direction::Up => grid_pos.y -= 1,
        Direction::Down => grid_pos.y += 1,
        Direction::Left => grid_pos.x -= 1,
        Direction::Right => grid_pos.x += 1
    }

    // Wrap Around
    grid_pos.x = (grid_pos.x + game_board.grid_size.0) % game_board.grid_size.0;
    grid_pos.y = (grid_pos.y + game_board.grid_size.1) % game_board.grid_size.1;
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

