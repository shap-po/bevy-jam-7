use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::util::room_switch::{ArrowDirection, arrow_room_switcher};
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BedroomBedAssets>();
    app.add_systems(
        Update,
        set_background
            .run_if(in_state(Room::BedroomBed))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct BedroomBedAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for BedroomBedAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/bedroom_bed.png"),
        }
    }
}

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::BedroomBed,
            forward_room: Some(Room::Bedroom),
            back_room: Some(Room::Bedroom),
            ..Default::default()
        },
        children![
            arrow_room_switcher(Room::Bedroom, ArrowDirection::Up),
        ],
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BedroomBedAssets>>,
) {
    sprite.set_image(&assets.background);
}
