use rand::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::game_board::board;
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
                if let Ok(next_position) = find_next_position(
                    &game_board,
                    &all_transforms
                        .iter()
                        .map(|t|{ game_board.world_pos_to_grid_pos(&t.translation) })
                        .collect::<Vec<GridPosition>>()
                ) {
                    spawn_food(next_position, &game_board, &mut commands);
                }
            }
        }
    });
}


fn find_next_position(
    game_board: &board::Desc,
    disallowed_positions: &[GridPosition]
) -> Result<GridPosition, String> {
    let max_positions = game_board.grid_size.0 * game_board.grid_size.1;
    if disallowed_positions.len() >= max_positions as usize {
        return Err(String::from("All positions disallowed"));
    }

    loop {
        let pos = GridPosition{
            x: random::<i32>().abs() % game_board.grid_size.0,
            y: random::<i32>().abs() % game_board.grid_size.1,
        };
        if !disallowed_positions.contains(&pos) {
            return Ok(pos);
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use super::*;

    fn init_plugin() -> App {
        let mut app = App::default();
        let board = board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        };
        app.world.insert_resource(board);
        let food_plugin = FoodPlugin{
            init_params: InitParams{
                start_position: GridPosition {x:0, y:0}
            }
        };
        food_plugin.build(&mut app);
        app.update();
        app
    }

    fn get_food_entity(app: &mut App) -> Entity {
        app.world
            .query::<(Entity, With<FoodComponent>)>()
            .iter(&app.world)
            .map(|(e, _)| e)
            .next()
            .unwrap()
    }

    fn consume_food(app: &mut App) {
        let food_entity = get_food_entity(app);
        app.world.resource_mut::<Events<ConsumeEvent>>().send(
            ConsumeEvent{
                target: food_entity
            });
        app.update();
    }

    #[test]
    fn find_next_position_fills_all() {
        let board = board::Desc{ grid_size: (2, 2), cell_size: 1 };
        let mut picked_positions = Vec::new();
        for _ in std::iter::repeat(()).take(4) {
            picked_positions.push(find_next_position(
                &board,
                &picked_positions
            ).unwrap());
        }
        assert_eq!(picked_positions.len(), 4);
        assert!(picked_positions.contains(&GridPosition{x:0, y:0}));
        assert!(picked_positions.contains(&GridPosition{x:1, y:0}));
        assert!(picked_positions.contains(&GridPosition{x:0, y:1}));
        assert!(picked_positions.contains(&GridPosition{x:1, y:1}));
    }

    #[test]
    fn find_next_position_fails_eventually() {
        let board = board::Desc{ grid_size: (2, 2), cell_size: 1 };
        let mut picked_positions = Vec::new();
        for _ in std::iter::repeat(()).take(4) {
            picked_positions.push(find_next_position(
                &board,
                &picked_positions
            ).unwrap());
        }

        assert_eq!(
            find_next_position(&board, &picked_positions),
            Err(String::from("All positions disallowed"))
        );
    }

    #[test]
    fn find_next_position_always_within_board() {
        let board = board::Desc{ grid_size: (30, 20), cell_size: 1 };
        let mut picked_positions = Vec::new();
        while let Ok(next_position) =
            find_next_position(&board, &picked_positions) {
            assert!(next_position.x >= 0);
            assert!(next_position.x < board.grid_size.0);
            assert!(next_position.y >= 0);
            assert!(next_position.y < board.grid_size.1);
            picked_positions.push(next_position);
        }
        assert_eq!(picked_positions.len(), (board.grid_size.0 * board.grid_size.1) as usize);
    }

    #[test]
    fn spawn_food_on_start() {
        let mut app = init_plugin();
        let food_count = app.world
            .query::<(Entity, With<FoodComponent>)>()
            .iter(&app.world)
            .count();
        assert_eq!(food_count, 1);
    }

    #[test]
    fn spawn_food_on_consume() {
        init_plugin();
        let mut app = init_plugin();
        let food_before = get_food_entity(&mut app);
        app.world.resource_mut::<Events<ConsumeEvent>>().send(
            ConsumeEvent{ target: food_before }
        );
        app.update();
        let food_after = get_food_entity(&mut app);
        assert_ne!(food_before, food_after);
    }

    #[test]
    fn always_only_one_food() {
        let mut app = init_plugin();

        consume_food(&mut app);
        consume_food(&mut app);
        consume_food(&mut app);
        consume_food(&mut app);

        let food_count = app.world
            .query::<(Entity, With<FoodComponent>)>()
            .iter(&app.world)
            .count();
        assert_eq!(food_count, 1);
    }

    #[test]
    fn consume_food_checks_correct_target() {
        let mut app = init_plugin();
        let food_before = get_food_entity(&mut app);
        let not_a_real_entity = Entity::from_raw(random::<u32>());
        app.world.resource_mut::<Events<ConsumeEvent>>().send(
            ConsumeEvent{ target: not_a_real_entity }
        );
        app.update();
        let food_after = get_food_entity(&mut app);
        assert_eq!(food_before, food_after);
    }
}
