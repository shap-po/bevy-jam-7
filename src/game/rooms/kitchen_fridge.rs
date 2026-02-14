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
    app.load_resource::<KitchenFridgeAssets>();
    app.add_systems(Startup, spawn_room);
    app.add_systems(Update, set_background.run_if(in_state(Room::KitchenFridge)));
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
            background: assets.load("images/splash.png"), // TODO: Set image
        }
    }
}

fn spawn_room(mut commands: Commands) {
    commands.spawn(room(
        "KitchenFridge",
        RoomComponent {
            this_room: Room::KitchenFridge,
            back_room: Some(Room::Bedroom),
            left_room: Some(Room::Bedroom),
            ..Default::default()
        },
        (),
    ));
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<KitchenFridgeAssets>>,
) {
    sprite.set_image(&assets.background);
}
