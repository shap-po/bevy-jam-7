use crate::game::{
    enemies::{Difficulty, Enemy, EnemyTicked, EnemyType},
    events::GameOver,
    rooms::Room,
};
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
    app.add_observer(washer_complete);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum Washer {
    Passive(i8),
    Active(i8),
    Death,
}

pub fn washer(difficulty: i8) -> impl Bundle {
    (
        Name::new("Washer"),
        Washer::Passive(MAXSTATES),
        Difficulty::new(difficulty),
        Enemy,
    )
}

const MAXSTATES: i8 = 10;

fn handle_opportunity(
    event: On<EnemyTicked>,
    washer_query: Single<(Entity, &mut Washer)>,
    mut command: Commands,
) {
    let (entity, mut washer) = washer_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *washer = match *washer {
        Washer::Passive(i) => {
            if i <= 0 {
                Washer::Active(MAXSTATES)
            } else {
                Washer::Passive(i - 1)
            }
        }
        Washer::Active(i) => {
            if i <= 0 {
                command.trigger(GameOver::Loose(EnemyType::Washer));
                Washer::Death
            } else {
                Washer::Active(i - 1)
            }
        }
        Washer::Death => Washer::Death,
    };
}

#[derive(Event, Reflect, Debug, Default)]
pub struct WashingMinigameComplete;

fn washer_complete(event: On<WashingMinigameComplete>, mut washer: Single<&mut Washer>) {
    **washer = Washer::Passive(MAXSTATES);
}
