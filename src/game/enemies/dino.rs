use bevy::{prelude::*, transform::components};

use crate::game::enemies::{Difficulty, Enemy, EnemyState};

pub(super) fn plugin(app: &mut App) {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Dino;

pub fn dino(difficulty: i8) -> impl Bundle {
    (
        Name::new("Dino"),
        Dino,
        Difficulty::new(difficulty),
        Enemy::new(),
    )
}
