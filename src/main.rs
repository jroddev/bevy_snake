use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prototype_lyon::prelude::*;
use std::collections::VecDeque;

const GRID_SIZE: (i32, i32) = (15, 15);
const CELL_SIZE: i32 = 15;
const START_POS: (i32, i32) = (0, 7);
const START_TAIL_LENGTH: usize = 3;
const TICK_TIME_SECONDS: f32 = 0.1;

fn main() {
    println!("Hello, Snake!");
    let tick_timer = GameplayTickTimer(
        Timer::from_seconds(TICK_TIME_SECONDS, true));
    let window_desc = WindowDescriptor {
        title: "Bevy Snake".to_string(),
        width: (GRID_SIZE.0 * CELL_SIZE) as f32,
        height: (GRID_SIZE.1 * CELL_SIZE) as f32,
        ..default()
    };

    let position_history : Vec<GridPosition> = vec![GridPosition{x:START_POS.0, y:START_POS.1}]
        .iter()
        .cycle()
        .take(START_TAIL_LENGTH)
        .cloned()
        .collect();
    let position_history: VecDeque<GridPosition> = VecDeque::from(position_history);

    App::new()
        .insert_resource(window_desc)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .insert_resource(tick_timer)
        .insert_resource(position_history)
        .add_system(tick_gameplay)
        .add_event::<TickEvent>()
        .add_startup_system(add_snake)
        .add_system(handle_input)
        .add_system(move_head)
        .add_system(snake_head_sprite_position)
        .add_system(snake_tail_sprite_positions)
        .add_system(draw_origin)
        .run();
}

fn draw_origin(mut commands: Commands){
    let width = (GRID_SIZE.0 * CELL_SIZE) as f32;
    for i in 0..GRID_SIZE.0 + 1 {
        let horizontal = shapes::Line(
            Vec2::new(0., -(i * CELL_SIZE)as f32),
            Vec2::new(width, -(i * CELL_SIZE)as f32)
        );
        commands.spawn_bundle(GeometryBuilder::build_as(
            &horizontal,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ));
    }

    for i in 0..GRID_SIZE.1 + 1 {
        let vertical = shapes::Line(
            Vec2::new((i * CELL_SIZE)as f32, 0.),
            Vec2::new((i * CELL_SIZE)as f32, -width)
        );
        commands.spawn_bundle(GeometryBuilder::build_as(
            &vertical,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(Color::BLACK, 1.0),
            },
            Transform::default(),
        ));
    }
}

fn add_snake(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = (CELL_SIZE * GRID_SIZE.0) as f32 * 0.5;
    camera.transform.translation.y = -((CELL_SIZE * GRID_SIZE.1) as f32 * 0.5);
    println!("camera: {},{}", camera.transform.translation.x, camera.transform.translation.y);
    commands.spawn_bundle(camera);

    commands
        .spawn()
        .insert(SnakeHead{})
        .insert(GridPosition {x: START_POS.0, y: START_POS.1})
        .insert(MovementController{direction: Direction::Right})
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        });

    for i in 0..START_TAIL_LENGTH {
        println!("create tail element {}", i);
        commands
            .spawn()
            .insert(SnakeTail{})
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(CELL_SIZE as f32, CELL_SIZE as f32)),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                ..default()
            });

    }
}

fn grid_pos_to_world_pos(grid_pos: &GridPosition) -> Vec3 {
    Vec3::new(
        (grid_pos.x * CELL_SIZE) as f32,
        -(grid_pos.y * CELL_SIZE) as f32,
        0.
    )
}

fn snake_head_sprite_position(mut query: Query<(&GridPosition, &mut Transform), With<SnakeHead>>) {
    if let Ok((grid_pos, mut transform)) = query.get_single_mut() {
        transform.translation = grid_pos_to_world_pos(grid_pos)
    }
}

fn snake_tail_sprite_positions(
    position_history: Res<VecDeque<GridPosition>>,
    mut query: Query<&mut Transform, With<SnakeTail>>) {
    for (index, mut transform) in query.iter_mut().enumerate() {
        if let Some(grid_pos) = &position_history.get(index) {
            transform.translation = grid_pos_to_world_pos(grid_pos);
        }
    }
}


fn tick_gameplay(
    time: Res<Time>,
    mut timer: ResMut<GameplayTickTimer>,
    mut tick_events: EventWriter<TickEvent>,) {
    if timer.0.tick(time.delta()).just_finished() {
        tick_events.send(TickEvent());
    }
}

fn move_head(
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
        grid_pos.x = (grid_pos.x + GRID_SIZE.0) % GRID_SIZE.0;
        grid_pos.y = (grid_pos.y + GRID_SIZE.1) % GRID_SIZE.1;
        // println!("position: {},{}", grid_pos.x, grid_pos.y);
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MovementController>
){
    let mut controller = query.single_mut();

    // TODO: reject turning immediately to the second piece
    if keyboard_input.just_pressed(KeyCode::Left) {
        controller.direction = Direction::Left;
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        controller.direction = Direction::Right;
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        controller.direction = Direction::Up;
    } else if  keyboard_input.just_pressed(KeyCode::Down) {
        controller.direction = Direction::Down;
    }
}

struct GameplayTickTimer(Timer);

#[derive(Default)]
struct TickEvent();

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct SnakeTail;


#[derive(Component, Clone)]
struct GridPosition { x: i32, y: i32 }

enum Direction { Up, Down, Left, Right }
#[derive(Component)]
struct MovementController { direction: Direction }

