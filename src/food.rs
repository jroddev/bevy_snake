use rand::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::game_board::board;
use crate::game_board::helpers::GameBoardHelpers;
use crate::core::GridPosition;

#[derive(Clone)]
pub struct InitParams{
    pub start_position: GridPosition
}

#[derive(Component)]
pub struct FoodComponent;

pub struct ConsumeEvent{
    pub(crate) target: Entity
}

pub struct FoodPlugin{
    pub init_params: InitParams
}


impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.init_params.clone())
            .add_event::<ConsumeEvent>()
            .add_startup_system(init_food)
            .add_system(consume_food);
    }
}

fn init_food(
    init_data: Res<InitParams>,
    game_board: Res<board::Desc>,
    mut commands: Commands
) {
    spawn_food(
        init_data.start_position.clone(),
        &game_board,
        &mut commands
    );
}


fn spawn_food(
    grid_position: GridPosition,
    game_board: &Res<board::Desc>,
    commands: &mut Commands
) {
    let translation = game_board
        .grid_pos_to_world_pos(&grid_position);

    commands
        .spawn()
        .insert(FoodComponent)
        .insert_bundle( SpriteBundle {
            transform: Transform { translation, ..default() },
            sprite: Sprite {
                color: Color::rgb(1., 1., 0.0),
                custom_size: Some(Vec2::new(
                    game_board.cell_size as f32,
                    game_board.cell_size as f32)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        });
}

fn consume_food(
    game_board: Res<board::Desc>,
    query: Query<Entity, With<FoodComponent>>,
    all_transforms: Query<&Transform>,
    mut consume_events: EventReader<ConsumeEvent>,
    mut commands: Commands
) {
    consume_events.iter().enumerate().for_each(|(_, event)|{
        match query.iter().find(|food|{*food == event.target}) {
            None => {}
            Some(food) => {
                commands.entity(food).despawn();
                spawn_food(
                    find_next_position(
                        &game_board,
                        all_transforms
                            .iter()
                            .map(|t|{ game_board.world_pos_to_grid_pos(&t.translation) })
                            .collect()
                    ),
                    &game_board,
                    &mut commands);
            }
        }
    });
}


fn find_next_position(
    game_board: &Res<board::Desc>,
    disallowed_positions: Vec<GridPosition>
) -> GridPosition {
    loop {
        let pos = GridPosition{
            x: random::<i32>().abs() % game_board.grid_size.0,
            y: random::<i32>().abs() % game_board.grid_size.1,
        };
        if !disallowed_positions.contains(&pos) {
            return pos;
        }
    }
}