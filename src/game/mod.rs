use bevy::prelude::*;

pub mod rooms;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((rooms::plugin,));
}

pub fn start_game(mut commands: Commands) {}
