use bevy::{prelude::*, transform::components};
use rand::prelude::*; //rand::random_bool(1.0)

use crate::game::enemies::{Difficulty, Enemy, EnemyTicked};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Dino{
    Away,
    Far,
    Stalk,
    Death,
}

#[rustfmt::skip]
pub fn dino(difficulty: i8) -> impl Bundle {
    (
        Name::new("Dino"),
        Dino::Away,
        Difficulty::new(difficulty),
        Enemy,
    )
}

fn handle_opportunity(_: On<EnemyTicked>, mut dino: Single<&mut Dino>, window: Res<KitchenWindowClosed>) {
    **dino = match **dino{
        Dino::Away => Dino::Far,
        Dino::Far => Dino::Stalk,
        Dino::Stalk => {
            if window.0 {Dino::Away}
            else {Dino::Death}
        },
        Dino::Death => todo!("manamded"),
    };
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct KitchenWindowClosed(bool);