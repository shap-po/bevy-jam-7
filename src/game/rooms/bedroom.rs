use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::Room;
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BedroomAssets>();
    // auto-enter the room once game starts
    app.add_systems(OnEnter(Screen::Gameplay), enter_room);
    // handle re-enter
    app.add_systems(
        OnEnter(Room::Bedroom),
        enter_room.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        handle_input
            .in_set(AppSystems::HandleInput)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay).and(in_state(Room::Bedroom))),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct BedroomAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for BedroomAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/ducky.png"),
        }
    }
}

fn enter_room(mut commands: Commands, room_assets: Res<BedroomAssets>) {
    commands.spawn((
        widget::ui_root("Bedroom"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Room::Bedroom),
        children![(widget::label("Bedroom"))],
    ));
}

fn handle_input(input: Single<&ActionState<Action>>, mut next_screen: ResMut<NextState<Room>>) {
    if input.just_pressed(&Action::Forward) {
        next_screen.set(Room::Hall)
    } else if input.just_pressed(&Action::Back) {
        next_screen.set(Room::Backroom)
    } else if input.just_pressed(&Action::Left) {
        next_screen.set(Room::Bathroom)
    } else if input.just_pressed(&Action::Right) {
        next_screen.set(Room::Kitchen)
    }
}
