use crate::game::{enemies::{Difficulty, Enemy, EnemyTicked}, rooms::Room};
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WashingMinigameComplete>();
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
enum Washer{
    Passive(i8),
    Active(i8),
    Death,
}

pub fn washer(difficulty: i8) -> impl Bundle {
    (
        Name::new("Washer"),
        Washer::Passive(0),
        Difficulty::new(difficulty),
        Enemy,
    )
}

const MAXSTATES: i8 = 10;

fn handle_opportunity(
    event: On<EnemyTicked>,
    washer_query: Single<(Entity, &mut Washer)>,
    minigame_complete : Res<WashingMinigameComplete>,
) {
    let (entity, mut washer) = washer_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *washer = match *washer {
        Washer::Passive(i) =>  {
            if i > MAXSTATES {
                Washer::Active(0)
            }
            else {
                Washer::Passive(i + 1)
            }
        },
        Washer::Active(i) => {
            if minigame_complete.0 {
                Washer::Passive(0)
            }
            else if i > MAXSTATES {
                Washer::Death
            }
            else {
                Washer::Passive(i + 1)
            }
        },
        Washer::Death => todo!(),
    };
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct WashingMinigameComplete(bool);