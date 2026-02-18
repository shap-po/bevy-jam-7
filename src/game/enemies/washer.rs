use crate::{
    audio::{PlaySfx, Sfxlib},
    game::{
        enemies::{Difficulty, Enemy, EnemyTicked, EnemyType},
        events::GameOver,
        rooms::Room,
    },
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
    app.add_observer(washer_complete);
}

const MAX_STATES_WORKING: i8 = 8;
const MAX_STATES_WAITING: i8 = 3;

#[derive(Component, Reflect, Debug, PartialEq)]
#[reflect(Component)]
pub enum Washer {
    Working(i8),
    Waiting(i8),
    Death,
}

impl Washer {
    fn working() -> Self {
        Self::Working(MAX_STATES_WORKING)
    }
    fn waiting() -> Self {
        Self::Waiting(MAX_STATES_WAITING)
    }
}

pub fn washer(difficulty: i8) -> impl Bundle {
    (
        Name::new("Washer"),
        Washer::working(),
        Difficulty(difficulty),
        Enemy::default(),
    )
}

fn handle_opportunity(
    event: On<EnemyTicked>,
    washer_query: Single<(Entity, &mut Washer)>,
    sfx_asset: Res<Sfxlib>,
    mut command: Commands,
) {
    let (entity, mut washer) = washer_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    *washer = match *washer {
        Washer::Working(i) => {
            if i <= 0 {
                command.play_loop_sfx(sfx_asset.washer_beep.clone(), 0.05, Room::BathroomWasher);
                Washer::waiting()
            } else {
                Washer::Working(i - 1)
            }
        }
        Washer::Waiting(i) => {
            if i <= 0 {
                //command.play_simple_sfx(sfx_asset.washer_outro.clone());
                command.play_volume_sfx(sfx_asset.boom.clone(), 0.1);
                command.trigger(GameOver::Death(EnemyType::Washer));
                Washer::Death
            } else {
                Washer::Waiting(i - 1)
            }
        }
        Washer::Death => Washer::Death,
    };
}

#[derive(Event, Reflect, Debug, Default)]
pub struct WashingMinigameComplete;

fn washer_complete(_: On<WashingMinigameComplete>, mut washer: Single<&mut Washer>) {
    **washer = Washer::working();
}
