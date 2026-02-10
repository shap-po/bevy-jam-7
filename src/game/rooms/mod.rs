use std::time::Duration;

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{AppSystems, PausableSystems, input_manager::Action, screens::Screen};

mod backroom;
mod bathroom;
mod bedroom;
mod hall;
mod kitchen;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Room>();

    app.add_plugins((
        backroom::plugin,
        bathroom::plugin,
        bedroom::plugin,
        hall::plugin,
        kitchen::plugin,
    ));
    app.add_systems(
        Update,
        go_back
            .in_set(AppSystems::HandleInput)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay).and(not(in_state(Room::Bedroom)))),
    );
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Room {
    #[default]
    Bedroom,
    Bathroom,
    Hall,
    Kitchen,
    Backroom,
}

fn go_back(input: Single<&ActionState<Action>>, mut next_screen: ResMut<NextState<Room>>) {
    if input.just_pressed(&Action::Back) {
        next_screen.set(Room::Bedroom);
    }
}
