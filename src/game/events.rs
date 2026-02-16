use bevy::{ecs::event, prelude::*};

use crate::game::enemies::EnemyType;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(death_event);
}

#[derive(Event, Debug, PartialEq)]
pub enum GameOver {
    Win,
    Death(EnemyType),
}

fn death_event(event: On<GameOver>) {
    println!("GameOver event received <= {:?}", *event);
}
