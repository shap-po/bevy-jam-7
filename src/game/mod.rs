use bevy::prelude::*;

mod enemies;
pub mod rooms;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        rooms::plugin,
        enemies::plugin,
    ));
}

pub fn start_game(mut commands: Commands) {}
