use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::time::Duration;
use std::collections::VecDeque;
use crate::game_board::GameBoardDesc;
use crate::core::GridPosition;

use super::head;
use super::tail;
use crate::food;

type WithAnySnakeType = Or<(With<head::SnakeHead>, With<tail::SnakeTail>)>;

#[derive(Clone)]
pub struct InitParams{
    pub movement_time_step: Duration,
    pub start_position: GridPosition,
    pub initial_tail_length: usize
}

pub fn add_snake(
    init_data: Res<InitParams>,
    game_board: Res<GameBoardDesc>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut commands: Commands
) {
    let starting_positions = VecDeque::from(
        fill_vec(
            GridPosition{x: -1, y: -1},
            init_data.initial_tail_length
        )
    );
    position_history.clear();
    position_history.clone_from(&starting_positions);


    head::spawn(&mut commands, init_data.start_position.clone(),game_board.cell_size as f32);

    for _ in 0..init_data.initial_tail_length {
        tail::spawn_node(&mut commands, game_board.cell_size as f32);
    }
}



pub fn get_snake_sprite_bundle(size: f32) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(size, size)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        ..default()
    }
}

pub fn cleanup_snake(
    query: Query<Entity, WithAnySnakeType>,
    mut commands: Commands
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn set_death_sprites(mut query: Query<&mut Sprite, Without<food::FoodComponent>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::RED;
    }
}



fn fill_vec<T: Clone>(value: T, count: usize) -> Vec<T> {
    vec![value]
        .iter()
        .cycle()
        .take(count)
        .cloned()
        .collect()
}
