use bevy::prelude::*;

use crate::{game::events::GameOver, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(change_screen);
}

fn change_screen(ev: On<GameOver>, mut next_screen: ResMut<NextState<Screen>>) {
    if *ev != GameOver::Win {
        return;
    }

    next_screen.set(Screen::Win);
}
