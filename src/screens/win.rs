use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles, game::events::GameOver, screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Win), spawn_win_screen);
    app.add_observer(change_screen);
}

fn change_screen(ev: On<GameOver>, mut next_screen: ResMut<NextState<Screen>>) {
    if *ev != GameOver::Win {
        return;
    }

    next_screen.set(Screen::Win);
}

fn spawn_win_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Win Screen"),
        GlobalZIndex(2),
        DespawnOnExit(Screen::Win),
        children![
            widget::header("You won!"),
            widget::button("Play again", enter_loading_or_gameplay_screen),
            widget::button("Quit", enter_title),
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn enter_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
