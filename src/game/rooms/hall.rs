use bevy::prelude::*;
use bevy::state::commands;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::enemies::doordash::Doordash;
use crate::game::rooms::{Background, Room, RoomComponent};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<HallAssets>();
    app.add_systems(Update, set_background.run_if(in_state(Room::Hall)));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct HallAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    background_w_hand: Handle<Image>,
}

impl FromWorld for HallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/hall_0.png"),
            background_w_hand: assets.load("images/rooms/hall_1.png"),
        }
    }
}

#[rustfmt::skip]
pub(super) fn room() -> impl Bundle {
    (
        RoomComponent {
            this_room: Room::Hall,
            back_room: Some(Room::Bedroom),
            ..Default::default()
        },
    )
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    doordash: Single<&Doordash>,
    assets: If<Res<HallAssets>>,
) {
    sprite.set_image(
        if let Doordash::Waiting {
            state_counter,
            food_want,
        } = *doordash
        {
            &assets.background_w_hand
        } else {
            &assets.background
        },
    );
}
