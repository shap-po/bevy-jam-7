use bevy::{prelude::*, transform::components};

use crate::{
    audio::{PlaySfx, Sfxlib},
    game::{
        enemies::{Difficulty, Enemy, EnemyTicked, EnemyType},
        events::GameOver,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<KitchenWindowClosed>();
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug, PartialEq)]
#[reflect(Component)]
pub enum Dino {
    Gone,
    Far,
    Near,
    Stalk,
    Death,
}

#[rustfmt::skip]
pub fn dino(difficulty: i8) -> impl Bundle {
    (
        Name::new("Dino"),
        Dino::Gone,
        Difficulty(difficulty),
        Enemy,
    )
}

fn handle_opportunity(
    event: On<EnemyTicked>,
    dino_query: Single<(Entity, &mut Dino)>,
    window: Res<KitchenWindowClosed>,
    sfx_asset: Res<Sfxlib>,
    mut command: Commands,
) {
    let (entity, mut dino) = dino_query.into_inner();
    if entity != event.event_target() {
        return;
    }
    *dino = match *dino {
        Dino::Gone => Dino::Far,
        Dino::Far => Dino::Near,
        Dino::Near => Dino::Stalk,
        Dino::Stalk => {
            if window.0 {
                command.play_volume_sfx(sfx_asset.rand_dino_gone(), 0.2);
                Dino::Gone
            } else {
                command.play_volume_sfx(sfx_asset.window_broke_in.clone(), 0.25);
                command.trigger(GameOver::Death(EnemyType::Dino));
                Dino::Death
            }
        }
        Dino::Death => Dino::Death,
    };
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct KitchenWindowClosed(pub bool);
