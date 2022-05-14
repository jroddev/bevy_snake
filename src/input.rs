use bevy::prelude::*;
use bevy::input::system::exit_on_esc_system;
use crate::core::Direction;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Direction>()
            .add_system(exit_on_esc_system)
            .add_system(handle_keyboard_input);
    }
}

fn handle_keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut direction_events: EventWriter<Direction>) {
    if keyboard_input.just_pressed(KeyCode::Left) {
            direction_events.send(Direction::Left)
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            direction_events.send(Direction::Right)
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            direction_events.send(Direction::Up)
        } else if keyboard_input.just_pressed(KeyCode::Down){
            direction_events.send(Direction::Down)
        }
}