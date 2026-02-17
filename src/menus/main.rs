//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles,
    audio::{PlaySfx, Sfxlib},
    game::save::GameState,
    menus::Menu,
    screens::Screen,
    theme::widget,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MainMenuAssets>();
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(OnEnter(Menu::Main), play_menu_music);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct MainMenuAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for MainMenuAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/house.png"),
        }
    }
}

fn spawn_main_menu(
    mut commands: Commands,
    game_state: Res<GameState>,
    assets: Res<MainMenuAssets>,
) {
    commands.spawn((
        (
            Name::new("Main Menu"),
            Node {
                position_type: PositionType::Absolute,
                width: percent(50),
                height: percent(100),
                left: percent(50),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                ..default()
            },
            Pickable::IGNORE,
        ),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        Sprite::from_image(assets.background.clone()),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button(
                format!("Play (night {})", game_state.night),
                enter_loading_or_gameplay_screen
            ),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button(
                format!("Play (night {})", game_state.night),
                enter_loading_or_gameplay_screen
            ),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
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

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn play_menu_music(mut commands: Commands, sfx_asset: Res<Sfxlib>) {
    commands.play_loop_sfx(sfx_asset.window_ambient_forest.clone(), 0.4, Menu::Main);
}
