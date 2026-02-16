use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameState>();
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct GameState {
    pub night: i8,
}

impl Default for GameState {
    fn default() -> Self {
        Self { night: 1 }
    }
}
