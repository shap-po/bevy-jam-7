use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BedroomAssets>();
    app.add_systems(
        Update,
        set_background
            .run_if(in_state(Room::Bedroom))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
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
            background: assets.load("images/rooms/bedroom.png"),
        }
    }
}

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::Bedroom,
            forward_room: Some(Room::Hall),
            back_room: Some(Room::BedroomBed),
            left_room: Some(Room::Bathroom),
            right_room: Some(Room::Kitchen),
            ..Default::default()
        },
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BedroomAssets>>,
) {
    sprite.set_image(&assets.background);
}
