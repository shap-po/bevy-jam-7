use bevy::{ecs::system::command, prelude::*};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<Action>::default());
    app.add_systems(Startup, spawn_input_map);
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    Forward,
    Back,
    Left,
    Right,
    Pause,
}

impl Action {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Forward, KeyCode::KeyW);
        input_map.insert(Self::Back, KeyCode::KeyS);
        input_map.insert(Self::Left, KeyCode::KeyA);
        input_map.insert(Self::Right, KeyCode::KeyD);
        input_map.insert(Self::Pause, KeyCode::Escape);

        input_map
    }
}

fn spawn_input_map(mut command: Commands) {
    command.spawn(Action::default_input_map());
}
