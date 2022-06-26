use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::time::Duration;
use crate::game_board::board;
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
    game_board: Res<board::Desc>,
    mut commands: Commands
) {
    let head = head::spawn(
        &mut commands,
        init_data.start_position.clone(),
        game_board.cell_size as f32
    );
    let mut follow_target = head;
    let tail_default_pos = GridPosition{x: -1, y: -1};
    for tail_index in 0..init_data.initial_tail_length {
        let tail = tail::spawn_node(
            &mut commands,
            tail_index,
            game_board.cell_size as f32,
            (follow_target, tail_default_pos.clone())
        );
        follow_target = tail;
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
        transform: Transform::identity().with_translation(Vec3::new(-size, -size, 0.)),
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
