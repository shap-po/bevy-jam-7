use bevy::prelude::*;
use bevy::state::commands;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::rooms::{Background, Room, RoomComponent, room};
use crate::input_manager::Action;
use crate::screens::Screen;
use crate::theme::widget;
use crate::utils::SetImage;
use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<KitchenAssets>();
    app.add_systems(Startup, spawn_room);
    app.add_systems(Update, set_background.run_if(in_state(Room::Kitchen)));
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
            background: assets.load("images/kitchen.png"),
        }
    }
}

fn spawn_room(mut commands: Commands) {
    commands.spawn(room(
        "Kitchen",
        RoomComponent {
            this_room: Room::Kitchen,
            back_room: Some(Room::Bedroom),
            forward_room: Some(Room::KitchenFridge),
            left_room: Some(Room::Bedroom),
            right_room: Some(Room::KitchenWindow),
            ..Default::default()
        },
        (),
    ));
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<KitchenAssets>>,
) {
    sprite.set_image(&assets.background);
}
