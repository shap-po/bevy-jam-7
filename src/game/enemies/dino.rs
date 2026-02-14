use bevy::{prelude::*, transform::components};
use rand::prelude::*; //rand::random_bool(1.0)

use crate::game::{enemies::{Difficulty, Enemy, EnemyTicked, EnemyType}, events::GameOver};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<KitchenWindowClosed>();
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Dino {
    Gone,
    Away,
    Far,
    Stalk,
    Death,
}

#[rustfmt::skip]
pub fn dino(difficulty: i8) -> impl Bundle {
    (
        Name::new("Dino"),
        Dino::Gone,
        Difficulty::new(difficulty),
        Enemy,
    )
}

fn handle_opportunity(
    event: On<EnemyTicked>,
    dino_query: Single<(Entity, &mut Dino)>,
    window: Res<KitchenWindowClosed>,
    mut command: Commands,
) {
    let (entity, mut dino) = dino_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *dino = match *dino {
        Dino::Gone => Dino::Away,
        Dino::Away => Dino::Far,
        Dino::Far => Dino::Stalk,
        Dino::Stalk => {
            if window.0 {
                Dino::Away
            } else {
                command.trigger(GameOver::Loose(EnemyType::Dino));
                Dino::Death
            }
        }
        Dino::Death => Dino::Death,
    };
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct KitchenWindowClosed(bool);
