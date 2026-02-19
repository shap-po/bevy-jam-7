use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::util::room_switch::room_switcher;
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
#[rustfmt::skip]
pub(super) struct BedroomAssets {
    #[dependency] background: Handle<Image>,
    #[dependency] left_door: Handle<Image>,
    #[dependency] middle_door: Handle<Image>,
    #[dependency] right_door: Handle<Image>,
}

impl FromWorld for BedroomAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/bedroom.png"),
            left_door: assets.load("images/rooms/prop/bedroom_door_left.png"),
            middle_door: assets.load("images/rooms/prop/bedroom_door_middle.png"),
            right_door: assets.load("images/rooms/prop/bedroom_door_right.png"),
        }
    }
}

#[rustfmt::skip]
pub(super) fn room(assets: &BedroomAssets) -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::Bedroom,
            forward_room: Some(Room::Hall),
            back_room: Some(Room::BedroomBed),
            left_room: Some(Room::Bathroom),
            right_room: Some(Room::Kitchen),
            ..Default::default()
        },
        children![
            room_switcher(Room::Bathroom, assets.left_door.clone()),
            room_switcher(Room::Hall, assets.middle_door.clone()),
            room_switcher(Room::Kitchen, assets.right_door.clone()),
        ]
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BedroomAssets>>,
) {
    sprite.set_image(&assets.background);
}
