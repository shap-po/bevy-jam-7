use bevy::prelude::*;

pub mod input;
pub mod rooms;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((input::plugin, rooms::plugin));
}

pub fn start_game(mut commands: Commands) {}
