use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::Room;
use crate::screens::Screen;
use crate::theme::widget;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HallAssets>();
    app.add_systems(
        OnEnter(Room::Hall),
        enter_room.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct HallAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for HallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/ducky.png"),
        }
    }
}

fn enter_room(mut commands: Commands, room_assets: Res<HallAssets>) {
    commands.spawn((
        widget::ui_root("Hall"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Room::Hall),
        children![(widget::label("Hall"))],
    ));
}
