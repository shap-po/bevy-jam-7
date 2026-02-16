use bevy::prelude::*;
use bevy::state::commands;
use leafwing_input_manager::prelude::ActionState;


use crate::asset_tracking::LoadResource;
use crate::audio::{PlaySfx, Sfxlib};
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};
use crate::game::enemies::washer::Washer;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BathroomAssets>();
    app.add_systems(Update, set_background.run_if(in_state(Room::Bathroom)));
    app.add_systems(OnEnter(Room::Bathroom), start_quet_washer_loop);
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
            background: assets.load("images/rooms/bathroom_0.png"),
        }
    }
}

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::Bathroom,
            forward_room: Some(Room::BathroomWasher),
            back_room: Some(Room::Bedroom),
            right_room: Some(Room::Bedroom),
            ..Default::default()
        },
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<BathroomAssets>>,
) {
    sprite.set_image(&assets.background);
}

fn start_quet_washer_loop(mut commands: Commands, sfx_asset: Res<Sfxlib>, washer: Single<&Washer>) {
    let Washer::Passive(_) = **washer else {
            return;
        };
    commands.play_loop_sfx(sfx_asset.washer_ambient_loop.clone(), 0.05, Room::Bathroom);
}