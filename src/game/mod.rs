use bevy::prelude::*;

use crate::screens::Screen;

pub mod enemies;
pub mod events;
mod interactions;
mod rooms;

#[rustfmt::skip]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        rooms::plugin,
        enemies::plugin,
        events::plugin,
        interactions::plugin,
    ));
}

pub fn start_game(mut commands: Commands) {
    commands.spawn((
        Name::new("Enemies"),
        DespawnOnExit(Screen::Gameplay),
        Transform::default(),
        Visibility::Inherited,
        children![
            enemies::dino::dino(10),
            enemies::ghost::ghost(10),
            enemies::doordash::doordash(10),
            enemies::washer::washer(10),
        ],
    ));
}
