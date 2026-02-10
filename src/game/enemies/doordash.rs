use crate::game::enemies::{Difficulty, Enemy, EnemyState};
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Doordash;

pub fn doordash(difficulty: i8) -> impl Bundle {
    (
        Name::new("Doordash"),
        Doordash,
        Difficulty::new(difficulty),
        Enemy::new(),
    )
}
