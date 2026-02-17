use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::game::enemies::dino::Dino;
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<KitchenAssets>();
    app.add_systems(
        Update,
        (set_background, block_movement)
            .run_if(in_state(Room::Kitchen))
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct KitchenAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    dino_death: Handle<Image>,
}

impl FromWorld for KitchenAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/kitchen.png"),
            dino_death: assets.load("images/rooms/kitchen_dino_death.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KitchenRoom;

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        KitchenRoom,
        RoomComponent {
            this_room: Room::Kitchen,
            back_room: Some(Room::Bedroom),
            forward_room: Some(Room::KitchenFridge),
            left_room: Some(Room::Bedroom),
            right_room: Some(Room::KitchenWindow),
            ..Default::default()
        },
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    dino: Single<&Dino>,
    assets: If<Res<KitchenAssets>>,
) {
    sprite.set_image(if **dino == Dino::Death {
        &assets.dino_death
    } else {
        &assets.background
    });
}

fn block_movement(mut room: Single<&mut RoomComponent, With<KitchenRoom>>, dino: Single<&Dino>) {
    if **dino == Dino::Death {
        room.can_move_forward = false;
        room.can_move_right = false;
    } else {
        room.unblock_movement();
    }
}
