use bevy::prelude::*;
use crate::core::GridPosition;
use crate::game_board::board;
use super::helpers;

#[derive(Component)]
pub struct SnakeTail{
    pub index: usize,
    pub follow_target: Entity,
    pub next_position: GridPosition
}

pub fn spawn_node(
    commands: &mut Commands,
    tail_index: usize,
    cell_size: f32,
    follow_target: (Entity, GridPosition)
) -> Entity {
    println!("spawn tail segment: {}", tail_index);
    commands
        .spawn()
        .insert(SnakeTail{
            index: tail_index,
            follow_target: follow_target.0,
            next_position: follow_target.1
        })
        .insert(GridPosition{ x: -1, y: -1})
        .insert_bundle(helpers::get_snake_sprite_bundle(cell_size))
        .id()
}

pub fn tick_position(
    game_board: Res<board::Desc>,
    mut tail_query: Query<(Entity, &mut Transform, &mut SnakeTail)>,
    mut grid_pos_query: Query<&mut GridPosition>,
) {
    for (tail_segment, mut transform, mut tail) in tail_query.iter_mut() {
        if let Ok(target_grid_pos) = grid_pos_query.get(tail.follow_target) {
            if target_grid_pos != &tail.next_position {
                let new_next_position = *target_grid_pos;
                if let Ok(mut current_grid_pos) = grid_pos_query.get_mut(tail_segment) {
                    transform.translation = game_board.grid_pos_to_world_pos(&tail.next_position);
                    current_grid_pos.set(&tail.next_position);
                    tail.next_position.set(&new_next_position);
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use rand::random;
    use super::*;

    #[derive(Clone, Copy)]
    struct TailParams {
        segment_index: usize,
        cell_size: f32,
        follow_target_grid_pos: GridPosition
    }

    fn test_system (
        tail_params: Res<TailParams>,
        mut commands: Commands) {

        let follow_target = commands
            .spawn()
            .insert(tail_params.follow_target_grid_pos)
            .id();

        spawn_node(
            &mut commands,
            tail_params.segment_index,
            tail_params.cell_size,
            (follow_target, tail_params.follow_target_grid_pos)
        );
    }


    #[test]
    fn spawn_node_creates_tail_entity() {
        let mut app = App::default();
        let tail_params = TailParams{
            segment_index: random::<usize>(),
            cell_size: random::<f32>(),
            follow_target_grid_pos: GridPosition{x: 2, y: 2}
        };
        app.insert_resource(tail_params);
        app.add_startup_system(test_system);
        app.update();
        let mut tail_query = app.world.query::<(
            Entity,
            &SnakeTail,
            &GridPosition,
            With<Sprite>)>();
        assert_eq!(tail_query.iter(&app.world).count(), 1);
        let (entity, tail, grid_pos, _) = tail_query.iter(&app.world).next().unwrap();
        assert_eq!(tail.index, tail_params.segment_index);
        assert_ne!(tail.follow_target, entity); // doesn't follow self
        assert_eq!(tail.next_position, tail_params.follow_target_grid_pos);
        assert_eq!(grid_pos, &GridPosition{x: -1, y: -1}); // initial grid positions is off screen

    }

    #[test]
    fn tick_position_does_nothing_before_target_moves() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        let tail_params = TailParams{
            segment_index: random::<usize>(),
            cell_size: random::<f32>(),
            follow_target_grid_pos: GridPosition{x: 2, y: 2}
        };
        app.insert_resource(tail_params);

        app.add_startup_system(test_system);
        app.add_system(tick_position);

        // Frame 1. Tail Segment Offscreen
        app.update();
        let ( tail_grid_pos, _) = app.world
            .query::<(&GridPosition, With<SnakeTail>)>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(tail_grid_pos, &GridPosition{x: -1, y: -1});

        let (mut follow_target_grid_pos, _) = app.world
            .query::<(&mut GridPosition, Without<SnakeTail>)>()
            .iter_mut(&mut app.world)
            .next()
            .unwrap();
        follow_target_grid_pos.x += 1;

        // Tail segment does not move if target stays in same place
        for _ in 0..5 {
            app.update();
            let ( tail_grid_pos, _) = app.world
                .query::<(&GridPosition, With<SnakeTail>)>()
                .iter(&app.world)
                .next()
                .unwrap();
            assert_eq!(tail_grid_pos, &tail_params.follow_target_grid_pos);
        }
    }

    #[test]
    fn tick_position_advances_grid_position_when_target_moves() {
        let mut app = App::default();
        app.world.insert_resource(board::Desc {
            grid_size: (5, 5),
            cell_size: 10
        });
        let tail_params = TailParams{
            segment_index: random::<usize>(),
            cell_size: random::<f32>(),
            follow_target_grid_pos: GridPosition{x: 2, y: 2}
        };
        app.insert_resource(tail_params);

        app.add_startup_system(test_system);
        app.add_system(tick_position);

        // Frame 1. Tail Segment Offscreen
        app.update();
        let ( tail_grid_pos, _) = app.world
            .query::<(&GridPosition, With<SnakeTail>)>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(tail_grid_pos, &GridPosition{x: -1, y: -1});

        let (mut follow_target_grid_pos, _) = app.world
            .query::<(&mut GridPosition, Without<SnakeTail>)>()
            .iter_mut(&mut app.world)
            .next()
            .unwrap();
        follow_target_grid_pos.x += 1;

        // Frame 2. Tail segment at initial follow target position
        app.update();
        let ( tail_grid_pos, _) = app.world
            .query::<(&GridPosition, With<SnakeTail>)>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(tail_grid_pos, &tail_params.follow_target_grid_pos);

        let (mut follow_target_grid_pos, _) = app.world
            .query::<(&mut GridPosition, Without<SnakeTail>)>()
            .iter_mut(&mut app.world)
            .next()
            .unwrap();
        follow_target_grid_pos.x += 1;

        // Frame 3. Tail segment at first moved follow target position
        app.update();
        let ( tail_grid_pos, _) = app.world
            .query::<(&GridPosition, With<SnakeTail>)>()
            .iter(&app.world)
            .next()
            .unwrap();
        assert_eq!(tail_grid_pos, &GridPosition{x: 3, y: 2});
    }



}