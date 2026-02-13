use crate::game::enemies::{Difficulty, Enemy, EnemyTicked};
use bevy::{prelude::*, transform::components};
use crate::game::rooms::Room;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Ghost(i8);

pub fn ghost(difficulty: i8) -> impl Bundle {
    (
        Name::new("Ghost"),
        Ghost::default(),
        Difficulty::new(difficulty),
        Enemy,
    )
}

fn handle_opportunity(_: On<EnemyTicked>, mut ghost: Single<&mut Ghost>, room: Res<State<Room>>){
    if **room == Room::Bedroom{
        ghost.0 = 0; 
    }
    else{
        ghost.0 += 1; 
        if ghost.0 > 10 { todo!("manamded")}
    }
}

