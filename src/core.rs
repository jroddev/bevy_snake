use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Direction { Up, Down, Left, Right }

#[derive(Component, Clone, PartialEq, Debug)]
pub struct GridPosition { pub x: i32, pub y: i32 }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    RUNNING,
    DEAD
}

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

        app
            .insert_resource(window_desc)
            .insert_resource(GameoverTimer(Timer::from_seconds(2.0, true)))
            .add_loopless_state(GameState::RUNNING)
            .add_enter_system(GameState::DEAD, start_game_over_timer)
            .add_system(start_new_game.run_in_state(GameState::DEAD));
    }
}


struct GameoverTimer(Timer);

fn start_game_over_timer(mut timer: ResMut<GameoverTimer>) {
    timer.0.reset();
}

fn start_new_game(
    time: Res<Time>,
    mut timer: ResMut<GameoverTimer>,
    mut commands: Commands
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.insert_resource(NextState(GameState::RUNNING));
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


