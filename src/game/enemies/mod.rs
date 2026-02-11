use std::time::Duration;

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems, screens::Screen};

pub(super) mod dino;
pub(super) mod doordash;
pub(super) mod washer;

const OPPORTUNITY_TIMER_DURATION: Duration = Duration::from_secs(5);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((dino::plugin, doordash::plugin, washer::plugin));
    app.insert_resource(EnemyOpportunity::new());
    app.add_systems(
        Update,
        enemy_tick
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct EnemyOpportunity(Timer);

impl EnemyOpportunity {
    fn new() -> Self {
        Self {
            0: Timer::new(OPPORTUNITY_TIMER_DURATION, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Difficulty)]
struct Enemy;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Difficulty(i8);

impl Difficulty {
    pub fn new(difficulty: i8) -> Self {
        Difficulty { 0: difficulty }
    }
}

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
