use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::input::MappedInput;
use crate::game::rooms::Room;
use crate::screens::Screen;
use crate::theme::widget;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<KitchenAssets>();
    app.add_systems(
        OnEnter(Room::Kitchen),
        enter_room.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct KitchenAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for KitchenAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/ducky.png"),
        }
    }
}

fn enter_room(mut commands: Commands, room_assets: Res<KitchenAssets>) {
    commands.spawn((
        widget::ui_root("Kitchen"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Room::Kitchen),
        children![(widget::label("Kitchen"))],
    ));
}
