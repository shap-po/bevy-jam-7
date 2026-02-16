use bevy::prelude::*;
use bevy::state::commands;
use bevy_bundled_observers::observers;
use leafwing_input_manager::prelude::ActionState;

use crate::asset_tracking::LoadResource;
use crate::game::enemies::doordash::{Doordash, FoodBrought};
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
pub struct HallAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    background_w_hand: Handle<Image>,
    #[dependency]
    door: Handle<Image>,
}

impl FromWorld for HallAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load("images/rooms/hall_0.png"),
            background_w_hand: assets.load("images/rooms/hall_1.png"),
            door: assets.load("images/rooms/prop/door.png"),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Door;

#[rustfmt::skip]
pub(super) fn room(assets:&HallAssets) -> impl Bundle {
    let mut sprite = Sprite::from_image(assets.door.clone());
    sprite.color.set_alpha(0.0);

    (
        RoomComponent {
            this_room: Room::Hall,
            back_room: Some(Room::Bedroom),
            ..Default::default()
        },
        children![
            (
                Door,
                Pickable::default(),
                sprite,
                observers![
                    give_food,
                ],
            ),
        ],
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

fn give_food(_: On<Pointer<Press>>, mut commands: Commands) {
    commands.trigger(FoodBrought);
}
