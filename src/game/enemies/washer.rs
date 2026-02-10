use bevy::{prelude::*, transform::components};
use crate::game::enemies::{Difficulty, Enemy, EnemyState};

pub(super) fn plugin(app: &mut App) {
    
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Washer;

pub fn washer(difficulty: i8) -> impl Bundle{
    (
        Name::new("Washer"),
        Washer,
        Difficulty::new(difficulty),
        Enemy::new(),
    )
}