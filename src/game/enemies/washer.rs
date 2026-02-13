use crate::game::enemies::{Difficulty, Enemy};
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Washer{
    Init,
    Passive(i8),
    Active(i8),
    Death,
}

pub fn washer(difficulty: i8) -> impl Bundle {
    (
        Name::new("Washer"),
        Washer::Init,
        Difficulty::new(difficulty),
        Enemy,
    )
}
