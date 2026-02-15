use crate::game::enemies::{Difficulty, Enemy, EnemyTicked, EnemyType, ghost_overlay};
use crate::game::events::GameOver;
use crate::game::rooms::Room;
use bevy::{prelude::*, transform::components};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_opportunity);
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
        Ghost::default(),
        Difficulty(difficulty),
        Enemy,
        children![
            ghost_overlay::overlay(),
        ]
    )
}

fn handle_opportunity(
    event: On<EnemyTicked>,
    ghost_query: Single<(Entity, &mut Ghost)>,
    room: Res<State<Room>>,
    mut command: Commands,
) {
    let (entity, mut ghost) = ghost_query.into_inner();
    if entity != event.event_target() {
        return;
    }

    if **room == Room::BedroomBed {
        ghost.0 = 0;
    } else {
        ghost.0 += 1;
        if ghost.0 >= MAX_STAGE {
            command.trigger(GameOver::Loose(EnemyType::Ghost));
            ghost.0 = MAX_STAGE;
        }
    }
}
