use bevy::prelude::*;
use iyes_loopless::prelude::*;
use std::collections::VecDeque;
use crate::core::{Direction, GameState, GridPosition};
use crate::game_board::GameBoardDesc;
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
            let reverse_direction = Direction::between(
                grid_pos,
                &position_history[position_history.len() - 1],
            );
            if new_direction != reverse_direction {
                // Prevent reversing directly onto itself
                controller.direction = new_direction;
            }
        }
    }
}

pub fn move_head(
    game_board: Res<GameBoardDesc>,
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
    game_board: Res<GameBoardDesc>,
    query: Query<(&GridPosition, With<head::SnakeHead>)>,
    mut consume_events: EventReader<food::ConsumeEvent>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut commands: Commands
) {
    if consume_events.iter().next().is_some() {
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
