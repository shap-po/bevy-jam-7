use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    game::{enemies::EnemyType, events::GameOver},
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DeathCause>();

    app.add_systems(OnEnter(Screen::Death), spawn_death_screen);
    app.add_observer(change_screen);
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
struct DeathCause(Option<EnemyType>);

fn change_screen(
    ev: On<GameOver>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut death_cause: ResMut<DeathCause>,
) {
    let GameOver::Death(enemy) = *ev else {
        return;
    };

    *death_cause = DeathCause(Some(enemy));
    next_screen.set(Screen::Death);
}

fn spawn_death_screen(mut commands: Commands, cause: Res<DeathCause>) {
    commands.spawn((
        widget::ui_root("Death Screen"),
        GlobalZIndex(2),
        DespawnOnExit(Screen::Death),
        children![
            widget::header("You died"),
            widget::label(if let Some(cause) = cause.0 {
                cause.get_cause()
            } else {
                "How did you do that?"
            }),
            widget::button("Try again", enter_loading_or_gameplay_screen),
            widget::button("Rage quit", enter_title),
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
