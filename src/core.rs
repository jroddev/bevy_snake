use bevy::prelude::*;


struct GameplayTickTimer(Timer);

#[derive(Default)]
pub struct TickEvent();

#[derive(PartialEq, Clone, Debug)]
pub enum Direction { Up, Down, Left, Right }

#[derive(Component, Clone, PartialEq, Debug)]
pub struct GridPosition { pub x: i32, pub y: i32 }

pub struct GameWindow {
    pub title: String,
    pub width: f32,
    pub height: f32,
}

pub struct GamePlugin{
    pub window: GameWindow,
    pub tick_time_seconds: f32
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let window_desc = WindowDescriptor {
            title: self.window.title.clone(),
            width: self.window.width,
            height: self.window.height,
            ..default()
        };
        let tick_timer = GameplayTickTimer(
            Timer::from_seconds(self.tick_time_seconds, true));

        app
            .insert_resource(window_desc)
            .insert_resource(tick_timer)
            .add_event::<TickEvent>()
            .add_system(tick_gameplay);
    }
}

fn tick_gameplay(
    time: Res<Time>,
    mut timer: ResMut<GameplayTickTimer>,
    mut tick_events: EventWriter<TickEvent>) {
    if timer.0.tick(time.delta()).just_finished() {
        tick_events.send(TickEvent());
    }
}

impl Direction {
    pub fn between(from: &GridPosition, to: &GridPosition) -> Direction {
        let x = to.x - from.x;
        let y = to.y - from.y;
        match (x, y) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Down,
            (0, -1) => Direction::Up,
            _ => Direction::Left
        }
    }
}


