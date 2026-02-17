use std::time::Duration;

use bevy::prelude::*;

use crate::{
    AppSystems, PausableSystems,
    game::{events::GameOver, save::GameState},
    screens::Screen,
};

pub mod enemies;
pub mod events;
mod interactions;
mod rooms;
pub mod save;

const GAME_DURATION: Duration = Duration::from_secs(5 * 60);

#[rustfmt::skip]
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>();

    app.add_plugins((
        rooms::plugin,
        enemies::plugin,
        events::plugin,
        interactions::plugin,
        save::plugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), reset_timer);
    app.add_systems(
        Update,
            tick_timer
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_systems(Update, totally_secret_night_selector);
}

struct DifficultyBundle {
    dino: i8,
    ghost: i8,
    doordash: i8,
    washer: i8,
}

pub fn start_game(mut commands: Commands, game_state: Res<GameState>) {
    let difficulty = match game_state.night {
        1 => DifficultyBundle {
            dino: 3,
            ghost: 2,
            doordash: 0,
            washer: 2,
        },
        2 => DifficultyBundle {
            dino: 6,
            ghost: 4,
            doordash: 2,
            washer: 4,
        },
        3 => DifficultyBundle {
            dino: 8,
            ghost: 10,
            doordash: 6,
            washer: 8,
        },
        _ => DifficultyBundle {
            dino: 15,
            ghost: 15,
            doordash: 15,
            washer: 15,
        },
    };

    commands.spawn((
        Name::new("Enemies"),
        DespawnOnExit(Screen::Gameplay),
        Transform::default(),
        Visibility::Inherited,
        children![
            enemies::dino::dino(difficulty.dino),
            enemies::ghost::ghost(difficulty.ghost),
            enemies::doordash::doordash(difficulty.doordash),
            enemies::washer::washer(difficulty.washer),
        ],
    ));
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct GameTimer(Timer);

impl Default for GameTimer {
    fn default() -> Self {
        Self(Timer::new(GAME_DURATION, TimerMode::Once))
    }
}

fn reset_timer(mut timer: ResMut<GameTimer>) {
    timer.0.reset();
    timer.0.unpause();
}

fn tick_timer(mut timer: ResMut<GameTimer>, time: Res<Time>, mut commands: Commands) {
    timer.0.tick(time.delta());
    if timer.0.is_finished() {
        commands.trigger(GameOver::Win);
    }
}

fn totally_secret_night_selector(
    mut game_state: ResMut<GameState>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Numpad1) {
        game_state.night = 1;
    } else if input.just_pressed(KeyCode::Numpad2) {
        game_state.night = 2;
    } else if input.just_pressed(KeyCode::Numpad3) {
        game_state.night = 3;
    } else if input.just_pressed(KeyCode::Numpad4) {
        game_state.night = 4;
    }
}
