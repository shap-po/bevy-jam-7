use crate::game::enemies::{Difficulty, Enemy, EnemyTicked};
use crate::game::rooms::Room;
use bevy::{prelude::*, transform::components};

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

fn handle_opportunity(
    event: On<EnemyTicked>,
    ghost_query: Single<(Entity, &mut Ghost)>,
    room: Res<State<Room>>,
) {
    let (entity, mut ghost) = ghost_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    if **room == Room::Bedroom {
        ghost.0 = 0;
    } else {
        ghost.0 += 1;
        if ghost.0 > 10 {
            todo!("manamded")
        }
    }
}
