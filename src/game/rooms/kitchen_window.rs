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
    app.load_resource::<KitchenWindowAssets>();
    app.add_systems(Startup, spawn_room);
    app.add_systems(Update, set_background.run_if(in_state(Room::KitchenWindow)));
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct KitchenWindowAssets {
    #[dependency]
    dino_gone: Handle<Image>,
    #[dependency]
    dino_far: Handle<Image>,
    #[dependency]
    dino_near: Handle<Image>,
    #[dependency]
    dino_stalk: Handle<Image>,
    #[dependency]
    closed_window: Handle<Image>,
}

impl FromWorld for KitchenWindowAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            dino_gone: assets.load("images/kitchen_dino_gone.png"),
            dino_far: assets.load("images/kitchen_dino_far.png"),
            dino_near: assets.load("images/kitchen_dino_near.png"),
            dino_stalk: assets.load("images/kitchen_dino_stalk.png"),
            closed_window: assets.load("images/kitchen_closed_window.png"),
        }
    }
}

fn spawn_room(mut commands: Commands) {
    commands.spawn(room(
        "KitchenWindow",
        RoomComponent {
            this_room: Room::KitchenWindow,
            back_room: Some(Room::Kitchen),
            left_room: Some(Room::KitchenFridge),
            ..Default::default()
        },
        (),
    ));
}

fn set_background(
    mut sprite: Single<&mut Sprite, With<Background>>,
    assets: If<Res<KitchenWindowAssets>>,
) {
    sprite.set_image(&assets.dino_gone);
}
