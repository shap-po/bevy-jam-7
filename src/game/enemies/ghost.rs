use crate::audio::{PlaySfx, Sfxlib};
use crate::game::enemies::{Difficulty, Enemy, EnemyTicked, EnemyType, ghost_overlay};
use crate::game::events::GameOver;
use crate::game::rooms::Room;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;
use bevy_bundled_observers::observers;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        add_advantage
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub const MAX_STAGE: i8 = 10;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Ghost(i8);

impl From<Ghost> for i8 {
    fn from(value: Ghost) -> Self {
        value.0
    }
}

#[rustfmt::skip]
pub fn ghost(difficulty: i8) -> impl Bundle {
    (
        Name::new("Ghost"),
        Transform::default(),
        Visibility::Inherited,
        Ghost::default(),
        Difficulty(difficulty),
        Enemy::default(),
        children![
            ghost_overlay::overlay(),
        ],
        observers![
            handle_opportunity,
        ],
    )
}

fn handle_opportunity(
    _: On<EnemyTicked>,
    mut ghost: Single<&mut Ghost>,
    room: Res<State<Room>>,
    mut command: Commands,
    sfx_assets: Res<Sfxlib>,
) {
    if **room == Room::BedroomBed {
        ghost.0 = 0;
    } else {
        ghost.0 += 1;
        if ghost.0 >= MAX_STAGE {
            command.play_volume_sfx(sfx_assets.freeze.clone(), 0.25);
            command.trigger(GameOver::Death(EnemyType::Ghost));
            ghost.0 = MAX_STAGE;
        }
    }
}

fn add_advantage(mut enemy: Single<&mut Enemy, With<Ghost>>, room: Res<State<Room>>) {
    enemy.has_advantage = **room == Room::BedroomBed;
}
