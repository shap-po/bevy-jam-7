use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::input::MappedInput;
use crate::game::rooms::Room;
use crate::screens::Screen;
use crate::theme::widget;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BackroomAssets>();
    app.add_systems(
        OnEnter(Room::Backroom),
        enter_room.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct BackroomAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for BackroomAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/ducky.png"),
        }
    }
}

fn enter_room(mut commands: Commands, room_assets: Res<BackroomAssets>) {
    commands.spawn((
        widget::ui_root("Backroom"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Room::Backroom),
        children![(widget::label("Backroom"))],
    ));
}
