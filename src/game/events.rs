use bevy::{ecs::event, prelude::*};

use crate::game::enemies::EnemyType;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(death_event);
}

#[derive(Event, Debug)]
pub enum GameOver {
    Win,
    Loose(EnemyType),
}

fn death_event(
    event: On<GameOver>,
){
    println!("GameOver event received <= {:?}", *event);
}