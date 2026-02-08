use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

mod dino;
mod doordash;
mod washer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((dino::plugin, doordash::plugin, washer::plugin));
    app.add_systems(
        Update,
        enemy_tick
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Difficulty)]
struct Enemy {
    state: EnemyState,
    timer: Timer,
}
#[derive(Reflect, Debug)]
enum EnemyState {
    Active,
    Inactive,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Difficulty(i8);

fn enemy_tick(mut enemy_query: Query<(&mut Enemy, &Difficulty)>, time: Res<Time>) {
    for (mut enemy, difficulty) in &mut enemy_query {
        if difficulty.0 == 0 {
            continue;
        }
        enemy.timer.tick(time.delta());
        if enemy.timer.is_finished() {
            match enemy.state {
                EnemyState::Active => {
                    todo!("MAKE PLAYER IMPLODE");
                }
                EnemyState::Inactive => {
                    enemy.timer.set_duration(Duration::from_secs(10));
                    enemy.state = EnemyState::Active;
                }
            }
        }
    }
}
