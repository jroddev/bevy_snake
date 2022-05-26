use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy::sprite::Anchor;
use std::collections::VecDeque;
use std::time::Duration;
use crate::core::{Direction, GamePhase, GridPosition};
use crate::food::{ConsumeEvent, FoodComponent};
use crate::game_board::{GameBoardDesc,GameBoardHelpers};

#[derive(Component)]
struct SnakeHead {}

#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct MovementController { direction: Direction }

type WithAnySnakeType = Or<(With<SnakeHead>, With<SnakeTail>)>;

#[derive(Clone)]
pub struct InitParams{
    pub movement_time_step: Duration,
    pub start_position: GridPosition,
    pub initial_tail_length: usize
}

pub struct SnakePlugin {
    pub init_params: InitParams
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct FixedUpdate;


fn fill_vec<T: Clone>(value: T, count: usize) -> Vec<T> {
    vec![value]
        .iter()
        .cycle()
        .take(count)
        .cloned()
        .collect()
}

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {


        let fixed_time_step = SystemStage::parallel()
            .with_system_set(ConditionSet::new()
                .run_in_state(GamePhase::RUNNING)
                .label("move")
                .with_system(move_head)
                .into())
            .with_system_set(
            ConditionSet::new()
                .run_in_state(GamePhase::RUNNING)
                .after("move")
                .with_system(consume_food)
                .with_system(check_collide_with_food)
                .with_system(check_for_bite_self)
                .into());

        app
            .insert_resource(VecDeque::<GridPosition>::new())
            .insert_resource(self.init_params.clone())
            .add_enter_system(GamePhase::RUNNING, add_snake)
            .add_exit_system(GamePhase::DEAD, cleanup_snake)
            .add_enter_system(GamePhase::DEAD, set_death_sprites)
            .add_stage_before(
                CoreStage::Update,
                FixedUpdate,
                FixedTimestepStage::new( self.init_params.movement_time_step)
                    .with_stage(fixed_time_step)
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GamePhase::RUNNING)
                    .with_system(handle_input)

                    .with_system(snake_head_sprite_position)
                    .with_system(snake_tail_sprite_positions)
                    .into()
            );
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

fn cleanup_snake(
    query: Query<Entity, WithAnySnakeType>,
    mut commands: Commands
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn set_death_sprites(mut query: Query<&mut Sprite, Without<FoodComponent>>) {
    for mut sprite in query.iter_mut() {
        sprite.color = Color::RED;
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
    mut head_query: Query<(&Transform, With<SnakeHead>)>,
    other_query: Query<((&Transform, Entity), With<FoodComponent>)>,
    mut consume_events: EventWriter<ConsumeEvent>,
) {
    if let Ok((head_transform, _)) = head_query.get_single_mut() {
        for ((transform, other_entity), _ ) in other_query.iter() {
            if head_transform.translation == transform.translation {
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
        // println!("history {:?}", position_history);
        commands
            .spawn()
            .insert(SnakeTail{})
            .insert_bundle(get_snake_sprite_bundle(game_board.cell_size as f32));
    }
}

fn check_for_bite_self(
    mut commands: Commands,
    query: Query<&GridPosition, With<SnakeHead>>,
    position_history: ResMut<VecDeque<GridPosition>>,
) {
    if let Ok(head) = query.get_single() {
        position_history
            .iter()
            .take(position_history.len()-1)
            .for_each(|pos|{
                if pos == head {
                    println!("dead at {:?}", pos);
                    println!("from {:?}", position_history);
                    // game_state.phase = GamePhase::DEAD;
                    commands.insert_resource(NextState(GamePhase::DEAD));
                }
            })
    }
}

fn move_head(
    game_board: Res<GameBoardDesc>,
    mut position_history: ResMut<VecDeque<GridPosition>>,
    mut query: Query<(&mut GridPosition, &MovementController, With<SnakeHead>)>
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
