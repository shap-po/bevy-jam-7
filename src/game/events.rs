use bevy::prelude::*;

use crate::game::enemies::EnemyType;

pub(super) fn plugin(app: &mut App) {}

#[derive(Event, Debug)]
enum GameOver {
    Win,
    Loose(EnemyType),
}
