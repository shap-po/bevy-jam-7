use bevy::{prelude::*, transform::components};

use crate::game::enemies::{Difficulty, Enemy, EnemyTicked};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Dino;

#[rustfmt::skip]
pub fn dino(difficulty: i8) -> impl Bundle {
    (
        Name::new("Dino"),
        Dino,
        Difficulty::new(difficulty),
        Enemy,
    )
}

fn handle_opportunity(_: On<EnemyTicked>, dino: Single<&mut Dino>) {
    todo!("Handle");
}
