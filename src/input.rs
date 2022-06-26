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
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        direction_events.send(Direction::Right)
    }
    if keyboard_input.just_pressed(KeyCode::Up) {
        direction_events.send(Direction::Up)
    }
    if keyboard_input.just_pressed(KeyCode::Down){
        direction_events.send(Direction::Down)
    }
}


#[cfg(test)]
mod tests {
    use bevy::ecs::event::Events;
    use super::*;

    fn init_system() -> App {
        let mut app = App::default();
        app.add_event::<bevy::input::keyboard::KeyboardInput>();
        app.add_plugin(GameInputPlugin);
        app.world.insert_resource(Input::<KeyCode>::default());
        app
    }

    fn get_direction_events(app: &App) -> Vec<Direction> {
        app.world
            .resource::<Events<Direction>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    #[test]
    fn up_key() {
        let mut app = init_system();
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Up);
        app.update();
        assert_eq!(get_direction_events(&app), vec![Direction::Up]);
    }

    #[test]
    fn down_key() {
        let mut app = init_system();
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Down);
        app.update();
        assert_eq!(get_direction_events(&app), vec![Direction::Down]);
    }

    #[test]
    fn left_key() {
        let mut app = init_system();
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Left);
        app.update();
        assert_eq!(get_direction_events(&app), vec![Direction::Left]);
    }

    #[test]
    fn right_key() {
        let mut app = init_system();
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Right);
        app.update();
        assert_eq!(get_direction_events(&app), vec![Direction::Right]);
    }

    #[test]
    fn multiple_keys() {
        let mut app = init_system();
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Right);
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Up);
        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Up);
        app.update();
        assert_eq!(get_direction_events(&app), vec![Direction::Right, Direction::Up]);
    }

    #[test]
    fn no_keys() {
        let mut app = init_system();
        app.update();
        assert_eq!(get_direction_events(&app), vec![]);
    }
}