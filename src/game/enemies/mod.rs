use std::time::Duration;

#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) mod dino;
pub(super) mod doordash;
pub(super) mod ghost;
mod ghost_overlay;
pub(super) mod washer;

const OPPORTUNITY_TIMER_DURATION: Duration = Duration::from_secs(5);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        dino::plugin,
        doordash::plugin,
        washer::plugin,
        ghost::plugin,
        ghost_overlay::plugin,
    ));
    app.init_resource::<EnemyOpportunity>();
    app.add_systems(
        Update,
        (
            #[cfg(debug_assertions)]
            dev_enemy_tick.run_if(input_just_pressed(KeyCode::KeyT)),
            enemy_tick,
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
struct EnemyOpportunity(Timer);

impl Default for EnemyOpportunity {
    fn default() -> Self {
        Self(Timer::new(OPPORTUNITY_TIMER_DURATION, TimerMode::Repeating))
    }
}

#[derive(Debug)]
pub enum EnemyType {
    Dino,
    Doordash,
    Ghost,
    Washer,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Difficulty)]
struct Enemy;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Difficulty(i8);

#[derive(EntityEvent, Debug)]
pub struct EnemyTicked {
    #[event_target]
    entity: Entity,
}

fn enemy_tick(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Enemy, &Difficulty)>,
    mut opportunity_timer: ResMut<EnemyOpportunity>,
    time: Res<Time>,
) {
    opportunity_timer.0.tick(time.delta());
    if !opportunity_timer.0.is_finished() {
        return;
    }

    for (entity, enemy, difficulty) in &mut enemy_query {
        if difficulty.0 == 0 {
            continue;
        }

        if rand::random_range(0.0..=20.0) < difficulty.0.into() {
            commands.trigger(EnemyTicked { entity });
        }
    }
}

#[cfg(debug_assertions)]
fn dev_enemy_tick(mut commands: Commands, mut enemy_query: Query<(Entity, &Enemy, &Difficulty)>) {
    for (entity, enemy, difficulty) in &mut enemy_query {
        if rand::random_range(0.0..=20.0) < difficulty.0.into() {
            commands.trigger(EnemyTicked { entity });
        }
    }
    println!("Dev ticked enemies");
}
