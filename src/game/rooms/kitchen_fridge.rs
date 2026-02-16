use bevy::prelude::*;
use bevy::state::commands;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::audio::{PlaySfx, Sfxlib};
use crate::game::enemies::dino::Dino;
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<KitchenFridgeAssets>();
    app.add_systems(
        Update,
        (set_background, force_move).run_if(in_state(Room::KitchenFridge)),
    );
    app.add_systems(OnEnter(Room::KitchenFridge), start_fridge_loop);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct KitchenFridgeAssets {
    #[dependency]
    background: Handle<Image>,
}

impl FromWorld for KitchenFridgeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/kitchen_fridge.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct KitchenFridgeRoom;

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        KitchenFridgeRoom,
        RoomComponent {
            this_room: Room::KitchenFridge,
            back_room: Some(Room::Kitchen),
            right_room: Some(Room::KitchenWindow),
            ..Default::default()
        },
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<KitchenFridgeAssets>>,
) {
    sprite.set_image(&assets.background);
}

fn force_move(dino: Single<&Dino>, mut next_room: ResMut<NextState<Room>>) {
    if **dino == Dino::Death {
        next_room.set(Room::Kitchen);
    }
}

fn start_fridge_loop(mut commands: Commands, sfx_asset: Res<Sfxlib>) {
    commands.play_loop_sfx(sfx_asset.fridge_loop.clone(), 0.02, Room::KitchenFridge);
}