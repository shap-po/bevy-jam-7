use bevy::{prelude::*, transform::components};
use crate::game::enemies::{Difficulty, Enemy, EnemyState};

pub(super) fn plugin(app: &mut App) {
    
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Doordash;

pub fn doordash(difficulty: i8) -> impl Bundle{
    (
        Name::new("Doordash"),
        Doordash,
        Difficulty::new(difficulty),
        Enemy::new(),
    )
}