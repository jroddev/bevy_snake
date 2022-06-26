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
        init_data.start_position,
        game_board.cell_size as f32
    );
    let mut follow_target = head;
    let tail_default_pos = GridPosition{x: -1, y: -1};
    for tail_index in 0..init_data.initial_tail_length {
        let tail = tail::spawn_node(
            &mut commands,
            tail_index,
            game_board.cell_size as f32,
            (follow_target, tail_default_pos)
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


#[cfg(test)]
mod tests {
    use rand::random;
    use crate::snake::head::SnakeHead;
    use crate::snake::tail::SnakeTail;
    use super::*;

    #[test]
    fn cleanup_snake_removes_all_head_and_tail() {
        let mut app = App::default();
        app.world.spawn().insert(SnakeHead{});
        app.world.spawn().insert(SnakeHead{});
        app.world.spawn().insert(SnakeTail{
            index: 1,
            follow_target: Entity::from_raw(1),
            next_position: GridPosition{x: -1, y: -1}
        });
        app.world.spawn().insert(SnakeTail{
            index: 2,
            follow_target: Entity::from_raw(2),
            next_position: GridPosition{x: -1, y: -1}
        });
        app.world.spawn().insert(SnakeTail{
            index: 3,
            follow_target: Entity::from_raw(2),
            next_position: GridPosition{x: -1, y: -1}
        });
        app.update();
        assert_eq!(app.world
            .query::<WithAnySnakeType>()
            .iter(&app.world)
            .count(), 5);

        app.add_system(cleanup_snake);
        app.update();
        assert_eq!(app.world
                       .query::<WithAnySnakeType>()
                       .iter(&app.world)
                       .count(), 0);
    }

    #[test]
    fn set_death_sprites_turns_sprite_red() {
        let mut app = App::default();
        app.world.spawn().insert(Sprite{color:Color::BLUE, ..default()});
        app.update();
        assert_eq!(app.world
            .query::<&Sprite>()
            .iter(&app.world)
            .next()
            .unwrap()
            .color, Color::BLUE);
        app.add_system(set_death_sprites);
        app.update();
        assert_eq!(app.world
                       .query::<&Sprite>()
                       .iter(&app.world)
                       .next()
                       .unwrap()
                       .color, Color::RED);
    }

    #[test]
    fn add_snake_creates_head_and_tail() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        let init_params = InitParams{
            movement_time_step: Default::default(),
            start_position: GridPosition::new(3, 3),
            initial_tail_length: 5
        };
        app.insert_resource(init_params.clone());
        app.add_startup_system(add_snake);
        app.update();

        assert_eq!(app.world
            .query::<&SnakeHead>()
            .iter(&app.world)
            .count(), 1
        );

        assert_eq!(app.world
                       .query::<&SnakeTail>()
                       .iter(&app.world)
                       .count(),
                   init_params.initial_tail_length
        );
    }

    #[test]
    fn add_snake_sets_up_follow_targets() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        app.insert_resource(InitParams{
            movement_time_step: Default::default(),
            start_position: GridPosition::new(3, 3),
            initial_tail_length: 2
        });
        app.add_startup_system(add_snake);
        app.update();

        let head_entity = app.world
            .query::<(Entity, With<SnakeHead>)>()
            .iter(&app.world)
            .map(|(e, _)| e)
            .next()
            .unwrap();

        let mut tail_segments = app.world
            .query::<(Entity, &SnakeTail)>()
            .iter(&app.world)
            .collect::<Vec<_>>();
        tail_segments.sort_by(|a, b| a.1.index.cmp(&b.1.index));

        assert_eq!(tail_segments[0].1.follow_target, head_entity);
        assert_eq!(tail_segments[1].1.follow_target, tail_segments[0].0);
    }

    #[test]
    fn get_snake_sprite_bundle_correct_size_and_color() {
        let size = random::<f32>().abs();
        let bundle = get_snake_sprite_bundle(size);
        assert_eq!(bundle.sprite.custom_size.unwrap().x, size);
        assert_eq!(bundle.sprite.custom_size.unwrap().y, size);
        assert_eq!(bundle.sprite.color, Color::rgb(0.25, 0.25, 0.75));
        assert!(std::matches!(bundle.sprite.anchor, Anchor::TopLeft));
    }
}