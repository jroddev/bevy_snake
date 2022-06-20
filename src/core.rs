use bevy::prelude::*;
use iyes_loopless::prelude::*;


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

impl Plugin for GameWindow {
    fn build(&self, app: &mut App) {
        let window_desc = WindowDescriptor {
            title: self.title.clone(),
            width: self.width,
            height: self.height,
            ..default()
        };

        app.insert_resource(window_desc);
    }
}

pub struct GameStatePlugin{
    pub tick_time_sec: f32,
    pub game_over_pause_sec: f32
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameOverTimer(Timer::from_seconds(self.game_over_pause_sec, true)))
            .add_loopless_state(GameState::RUNNING)
            .add_enter_system(GameState::DEAD, start_game_over_timer)
            .add_system(start_new_game.run_in_state(GameState::DEAD));
    }
}


struct GameOverTimer(Timer);

fn start_game_over_timer(mut timer: ResMut<GameOverTimer>) {
    timer.0.reset();
}

fn start_new_game(
    time: Res<Time>,
    mut timer: ResMut<GameOverTimer>,
    mut commands: Commands
) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.insert_resource(NextState(GameState::RUNNING));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn switch_from_dead_to_running_after_time() {
        let mut app = App::default();
        app.add_plugins(MinimalPlugins);
        app.add_plugin( GameStatePlugin{ tick_time_sec: 1.0, game_over_pause_sec: 1.0 });
        app.update(); // setup initial state
        assert_eq!(app.world.resource::<CurrentState<GameState>>().0, GameState::RUNNING);
        app.world.insert_resource(NextState(GameState::DEAD));
        app.update(); // process state change
        assert_eq!(app.world.resource::<CurrentState<GameState>>().0, GameState::DEAD);
        thread::sleep(Duration::from_millis(500));
        app.update(); // tick timer
        assert_eq!(app.world.resource::<CurrentState<GameState>>().0, GameState::DEAD);
        thread::sleep(Duration::from_millis(500));
        app.update(); // tick + complete timer
        app.update(); // process state change
        assert_eq!(app.world.resource::<CurrentState<GameState>>().0, GameState::RUNNING);
    }
}