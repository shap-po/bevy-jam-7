use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::Room;
use crate::screens::Screen;
use crate::theme::widget;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BathroomAssets>();
    app.add_systems(
        OnEnter(Room::Bathroom),
        enter_room.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct BathroomAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for BathroomAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/ducky.png"),
        }
    }
}

fn enter_room(mut commands: Commands, room_assets: Res<BathroomAssets>) {
    commands.spawn((
        widget::ui_root("Bathroom"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Room::Bathroom),
        children![(widget::label("Bathroom"))],
    ));
}
