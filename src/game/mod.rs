use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, game::events::GameOver, screens::Screen};

pub mod enemies;
pub mod events;
mod interactions;
mod rooms;

const GAME_DURATION: Duration = Duration::from_secs(5 * 60);

#[rustfmt::skip]
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameTimer>();

    app.add_plugins((
        rooms::plugin,
        enemies::plugin,
        events::plugin,
        interactions::plugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), reset_timer);
    app.add_systems(
        Update,
        (
            #[cfg(debug_assertions)]
            tick_timer,
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

pub fn start_game(mut commands: Commands) {
    commands.spawn((
        Name::new("Enemies"),
        DespawnOnExit(Screen::Gameplay),
        Transform::default(),
        Visibility::Inherited,
        children![
            enemies::dino::dino(10),
            enemies::ghost::ghost(10),
            enemies::doordash::doordash(10),
            enemies::washer::washer(10),
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
