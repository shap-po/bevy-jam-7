use bevy::{ecs::name, prelude::*};

use crate::screens::Screen;

mod enemies;
pub mod events;
mod interactions;
mod rooms;

#[rustfmt::skip]
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        rooms::plugin,
        enemies::plugin,
        interactions::plugin,
    ));
}

pub fn start_game(mut commands: Commands) {
    commands.spawn((
        Name::new("Enemies"),
        DespawnOnExit(Screen::Gameplay),
        children![enemies::dino::dino(10),],
    ));
}
