use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::collections::VecDeque;
use crate::core::{Direction, GridPosition, TickEvent};
use crate::food::{ConsumeEvent, FoodComponent};
use crate::game_board::{GameBoardDesc,GameBoardHelpers};

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct MovementController { direction: Direction }

#[derive(Clone)]
pub struct InitParams{
    pub start_position: GridPosition,
    pub initial_tail_length: usize
}

pub struct SnakePlugin {
    pub init_params: InitParams
}


impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        let position_history : Vec<GridPosition> = vec![GridPosition{
            x: self.init_params.start_position.x,
            y: self.init_params.start_position.y
        }]
            .iter()
            .cycle()
            .take(self.init_params.initial_tail_length)
            .cloned()
            .collect();
        let position_history: VecDeque<GridPosition> = VecDeque::from(position_history);

        app
            .insert_resource(position_history)
            .insert_resource(self.init_params.clone())
            .add_startup_system(add_snake)
            .add_system(handle_input)
            .add_system(move_head)
            .add_system(consume_food)
            .add_system(check_collide_with_food)
            .add_system(snake_head_sprite_position)
            .add_system(snake_tail_sprite_positions);
    }
}


fn get_snake_sprite_bundle(size: f32) -> SpriteBundle {
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


fn add_snake(
    init_data: Res<InitParams>,
    game_board: Res<GameBoardDesc>,
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = (game_board.cell_size * game_board.grid_size.0) as f32 * 0.5;
    camera.transform.translation.y = -((game_board.cell_size * game_board.grid_size.1) as f32 * 0.5);
    commands.spawn_bundle(camera);

    commands
        .spawn()
        .insert(SnakeHead{})
        .insert(GridPosition {
            x: init_data.start_position.x,
            y: init_data.start_position.y
        })
        .insert(MovementController{direction: Direction::Right})
        .insert_bundle(get_snake_sprite_bundle(game_board.cell_size as f32));

    for _ in 0..init_data.initial_tail_length {
        commands
            .spawn()
            .insert(SnakeTail{})
            .insert_bundle(get_snake_sprite_bundle(game_board.cell_size as f32));
    }
}


fn snake_head_sprite_position(
    game_board: Res<GameBoardDesc>,
    mut query: Query<(&GridPosition, &mut Transform), With<SnakeHead>>
) {
    if let Ok((grid_pos, mut transform)) = query.get_single_mut() {
        transform.translation = game_board.grid_pos_to_world_pos(grid_pos)
    }
}

fn snake_tail_sprite_positions(
    game_board: Res<GameBoardDesc>,
    position_history: Res<VecDeque<GridPosition>>,
    mut query: Query<&mut Transform, With<SnakeTail>>
) {
    for (index, mut transform) in query.iter_mut().enumerate() {
        if let Some(grid_pos) = &position_history.get(index) {
            transform.translation = game_board.grid_pos_to_world_pos(grid_pos);
        }
    }
}

fn check_collide_with_food(
    head_query: Query<(&Transform, With<SnakeHead>)>,
    other_query: Query<((&Transform, Entity), With<FoodComponent>)>,
    mut consume_events: EventWriter<ConsumeEvent>,
) {
    if let Ok(head) = head_query.get_single() {
        for ((transform, other_entity), _ ) in other_query.iter() {
            if head.0.translation == transform.translation {
                consume_events.send(ConsumeEvent{target: other_entity});
                return;
            }
        }
    }
}

fn consume_food(
    game_board: Res<GameBoardDesc>,
    query: Query<(&GridPosition, With<SnakeHead>)>,
    mut consume_events: EventReader<ConsumeEvent>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut commands: Commands
) {
    if consume_events.iter().next().is_some() {

        let (grid_pos, _) = query.single();
        position_history.push_back(grid_pos.clone());
        commands
            .spawn()
            .insert(SnakeTail{})
            .insert_bundle(get_snake_sprite_bundle(game_board.cell_size as f32));
    }
}

fn move_head(
    game_board: Res<GameBoardDesc>,
    mut tick_events: EventReader<TickEvent>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut query: Query<(&mut GridPosition, &MovementController, With<SnakeHead>)>
){
    if tick_events.iter().count() > 0 {
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
}

fn handle_input(
    position_history: ResMut<VecDeque<GridPosition>>,
    mut direction_events: EventReader<Direction>,
    mut query: Query<(&GridPosition,&mut MovementController), With<SnakeHead>>
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
